use crate::core::evaluate_service::EvaluateService;
use crate::view::repl::ReplView;
use crate::view::viewable::Viewable;

pub struct InlineView {
    pub(crate) expression: String
}

impl Viewable for InlineView {
    fn run(&self) {
        let mut service = EvaluateService::new();
        let res = service.evaluate(&self.expression);

        ReplView::print_result(res);
    }
}