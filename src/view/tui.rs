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
    cursor_pos: usize,
    saved_input: String,
    history: Vec<Line<'static>>,
    history_scroll: u16,
    history_viewport_height: u16,
    plot: Option<Vec<(f64, f64)>>,
    cmd_history: Vec<String>,
    cmd_cursor: Option<usize>,
}

impl App {
    fn move_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    fn move_right(&mut self) {
        self.cursor_pos = (self.cursor_pos + 1).min(self.input.chars().count());
    }

    fn insert_char(&mut self, c: char) {
        let byte_idx = self.input.char_indices()
            .nth(self.cursor_pos)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(byte_idx, c);
        self.cursor_pos += 1;
    }

    fn delete_char_before(&mut self) {
        if self.cursor_pos == 0 { return; }
        let byte_idx = self.input.char_indices()
            .nth(self.cursor_pos - 1)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.remove(byte_idx);
        self.cursor_pos -= 1;
    }

    fn set_input(&mut self, s: String) {
        self.cursor_pos = s.chars().count();
        self.input = s;
    }

    fn history_up(&mut self) {
        if self.cmd_history.is_empty() { return; }
        match self.cmd_cursor {
            None => {
                self.saved_input = self.input.clone();
                self.cmd_cursor = Some(self.cmd_history.len() - 1);
            }
            Some(0) => return,
            Some(n) => self.cmd_cursor = Some(n - 1),
        }
        self.set_input(self.cmd_history[self.cmd_cursor.unwrap()].clone());
    }

    fn history_down(&mut self) {
        match self.cmd_cursor {
            None => {}
            Some(n) if n + 1 >= self.cmd_history.len() => {
                self.cmd_cursor = None;
                self.set_input(self.saved_input.clone());
            }
            Some(n) => {
                self.cmd_cursor = Some(n + 1);
                self.set_input(self.cmd_history[n + 1].clone());
            }
        }
    }

    fn submit(&mut self) {
        let input = self.input.trim().to_string();
        self.input.clear();
        self.cursor_pos = 0;
        self.cmd_cursor = None;
        self.saved_input.clear();

        if input.is_empty() {
            return;
        }

        self.cmd_history.push(input.clone());
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
            cursor_pos: 0,
            saved_input: String::new(),
            history: Vec::new(),
            history_scroll: 0,
            history_viewport_height: 0,
            plot: None,
            cmd_history: Vec::new(),
            cmd_cursor: None,
        };

        loop {
            terminal.draw(|frame| draw(frame, &mut app)).unwrap();

            match event::read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => app.submit(),
                    KeyCode::Backspace => app.delete_char_before(),
                    KeyCode::Left => app.move_left(),
                    KeyCode::Right => app.move_right(),
                    KeyCode::Up => app.history_up(),
                    KeyCode::Down => app.history_down(),
                    KeyCode::Char(c) => app.insert_char(c),
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

    let mut h_zero: Vec<(f64, f64)> = vec![];
    let mut v_zero: Vec<(f64, f64)> = vec![];
    let block = Block::new().borders(Borders::ALL).title("plot");
    let chart = match &app.plot {
        Some(points) if !points.is_empty() => {
            let x_min = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
            let x_max = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
            let y_min = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
            let y_max = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
            let y_pad = ((y_max - y_min) * 0.1).max(1.0);
            let y_bounds = [y_min - y_pad, y_max + y_pad];

            h_zero = vec![(x_min, 0.0), (x_max, 0.0)];
            v_zero = vec![(0.0, y_bounds[0]), (0.0, y_bounds[1])];

            let curve = Dataset::default()
                .data(points)
                .graph_type(GraphType::Line)
                .style(Style::new().fg(Color::Green));

            let mut datasets = vec![curve];
            if y_bounds[0] <= 0.0 && 0.0 <= y_bounds[1] {
                datasets.push(Dataset::default()
                    .data(&h_zero)
                    .graph_type(GraphType::Line)
                    .style(Style::new().fg(Color::DarkGray)));
            }
            if x_min <= 0.0 && 0.0 <= x_max {
                datasets.push(Dataset::default()
                    .data(&v_zero)
                    .graph_type(GraphType::Line)
                    .style(Style::new().fg(Color::DarkGray)));
            }

            Chart::new(datasets)
                .block(block)
                .x_axis(Axis::default()
                    .bounds([x_min, x_max])
                    .labels(vec![
                        Span::from(format!("{x_min:.1}")),
                        Span::from(format!("{x_max:.1}")),
                    ]))
                .y_axis(Axis::default()
                    .bounds(y_bounds)
                    .labels(vec![
                        Span::from(format!("{:.1}", y_bounds[0])),
                        Span::from(format!("{:.1}", y_bounds[1])),
                    ]))
        }
        _ => Chart::new(vec![]).block(block),
    };
    frame.render_widget(chart, plot_area);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::new().borders(Borders::ALL).title("input"));

    frame.render_widget(input, input_area);

    let cursor_x = input_area.x + 1 + app.cursor_pos as u16;
    let cursor_y = input_area.y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    let memory_lines: Vec<Line> = app
        .service
        .identifiers_registry()
        .user_entries()
        .into_iter()
        .map(|(name, value)| Line::from(format!("{name} = {value}")))
        .collect();

    let memory = Paragraph::new(memory_lines)
        .block(Block::new().borders(Borders::ALL).title("memory"));

    frame.render_widget(memory, memory_area);
}

