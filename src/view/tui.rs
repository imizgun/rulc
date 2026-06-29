use crate::core::evaluate_service::EvaluateService;
use crate::core::repl_output::ReplOutput;
use crate::core::runtime_error::RuntimeError;
use crate::view::viewable::Viewable;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph};

pub struct TuiView;

struct App {
    service: EvaluateService,
    input: String,
    history: Vec<Line<'static>>,
    history_scroll: u16,
    history_viewport_height: u16,
    plot: Option<Vec<(f64, f64)>>,
}

impl App {
    fn submit(&mut self) {
        let input = self.input.trim().to_string();
        self.input.clear();

        if input.is_empty() {
            return;
        }

        self.history.push(Line::from(format!("{INPUT_PREFIX}{input}")));

        match self.service.evaluate(&input) {
            Ok(ReplOutput::FuncPoints { points }) => {
                self.plot = Some(points);
                self.history.push(
                    Line::from(format!("{INPUT_PREFIX}plot updated"))
                        .style(Style::new().fg(Color::Cyan)),
                );
            }
            other => self.history.extend(format_result(other)),
        }

        self.history_scroll = self.max_scroll();
    }

    fn max_scroll(&self) -> u16 {
        (self.history.len() as u16).saturating_sub(self.history_viewport_height)
    }

    fn scroll_by(&mut self, delta: i16) {
        let scroll = self.history_scroll as i16 + delta;
        self.history_scroll = scroll.clamp(0, self.max_scroll() as i16) as u16;
    }
}

const INPUT_PREFIX: &str = ">> ";
const OUTPUT_INDENT: &str = "   ";

fn format_result(res: Result<ReplOutput, RuntimeError>) -> Vec<Line<'static>> {
    match res {
        Ok(out) => vec![
            Line::from(format!("{INPUT_PREFIX}{out}"))
                .style(Style::new().fg(Color::Green)),
        ],
        Err(err) => {
            let lines = err.display_lines();
            vec![
                Line::from(format!("{INPUT_PREFIX}{}", lines.formatted_tokens))
                    .style(Style::new().fg(Color::Red)),
                Line::from(format!("{OUTPUT_INDENT}{}", lines.error))
                    .style(Style::new().fg(Color::Red)),
            ]
        }
    }
}

impl Viewable for TuiView {
    fn run(&self) {
        let mut terminal = ratatui::init();
        crossterm::execute!(std::io::stdout(), EnableMouseCapture).unwrap();

        let mut app = App {
            service: EvaluateService::new(),
            input: String::new(),
            history: Vec::new(),
            history_scroll: 0,
            history_viewport_height: 0,
            plot: None,
        };

        loop {
            terminal.draw(|frame| draw(frame, &mut app)).unwrap();

            match event::read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => app.submit(),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Char(c) => app.input.push(c),
                    _ => {}
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollUp => app.scroll_by(-1),
                    MouseEventKind::ScrollDown => app.scroll_by(1),
                    _ => {}
                },
                _ => {}
            }
        }

        crossterm::execute!(std::io::stdout(), DisableMouseCapture).unwrap();
        ratatui::restore();
    }
}

fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    let [plot_area, bottom_area] =
        Layout::vertical([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
            .areas(frame.area());

    let [repl_area, memory_area] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .areas(bottom_area);

    let [history_area, input_area] =
        Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)])
            .areas(repl_area);

    app.history_viewport_height = history_area.height.saturating_sub(2);
    app.history_scroll = app.history_scroll.min(app.max_scroll());

    let history = Paragraph::new(app.history.clone())
        .block(Block::new().borders(Borders::ALL).title("history"))
        .scroll((app.history_scroll, 0));

    frame.render_widget(history, history_area);
    frame.render_widget(build_chart(&app.plot), plot_area);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::new().borders(Borders::ALL).title("input"));

    frame.render_widget(input, input_area);

    let cursor_x = input_area.x + 1 + app.input.chars().count() as u16;
    let cursor_y = input_area.y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    let memory_lines: Vec<Line> = app
        .service
        .identifiers_registry()
        .entries()
        .into_iter()
        .map(|(name, value)| Line::from(format!("{name} = {value}")))
        .collect();

    let memory = Paragraph::new(memory_lines)
        .block(Block::new().borders(Borders::ALL).title("memory"));

    frame.render_widget(memory, memory_area);
}

fn build_chart(plot: &Option<Vec<(f64, f64)>>) -> Chart<'_> {
    let block = Block::new().borders(Borders::ALL).title("plot");

    let Some(points) = plot else {
        return Chart::new(vec![]).block(block);
    };

    if points.is_empty() {
        return Chart::new(vec![]).block(block);
    }

    let x_min = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let x_max = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let y_min = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let y_max = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let y_pad = ((y_max - y_min) * 0.1).max(1.0);
    let y_bounds = [y_min - y_pad, y_max + y_pad];

    let curve = Dataset::default()
        .data(points)
        .graph_type(GraphType::Line)
        .style(Style::new().fg(Color::Green));

    Chart::new(vec![curve])
        .block(block)
        .x_axis(
            Axis::default()
                .bounds([x_min, x_max])
                .labels(vec![
                    Span::from(format!("{x_min:.1}")),
                    Span::from(format!("{x_max:.1}")),
                ]),
        )
        .y_axis(
            Axis::default()
                .bounds(y_bounds)
                .labels(vec![
                    Span::from(format!("{:.1}", y_bounds[0])),
                    Span::from(format!("{:.1}", y_bounds[1])),
                ]),
        )
}