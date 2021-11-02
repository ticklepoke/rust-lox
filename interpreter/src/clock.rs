use frontend::{callable::Callable, instance::Instance, literal::Literal};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut dyn frontend::runnable::Runnable,
        _args: Vec<frontend::literal::Literal>,
    ) -> frontend::scanner::ScannerResult<frontend::literal::Literal> {
        Ok(Literal::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Unable to get time")
                .as_millis() as f64,
        ))
    }

    fn box_clone(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    fn bind(&self, _instance: Instance) -> Box<dyn Callable> {
        // HACK noop
        self.box_clone()
    }
}
