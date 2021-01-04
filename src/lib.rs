use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(SampleFilterRoot {
            req_metric_id: 0,
            resp_metric_id: 0,
        })
    });
}

struct SampleFilterRoot {
    req_metric_id: u32,
    resp_metric_id: u32,
}

impl Context for SampleFilterRoot {}

impl RootContext for SampleFilterRoot {
    fn on_vm_start(&mut self, _: usize) -> bool {
        self.req_metric_id = match proxy_wasm::hostcalls::define_metric(
            MetricType::Counter,
            "stat_filter.custom_metric.num_of_request",
        ) {
            Ok(metric_id) => metric_id,
            Err(e) => panic!("Error: {:?}", e),
        };

        self.resp_metric_id = match proxy_wasm::hostcalls::define_metric(
            MetricType::Counter,
            "stat_filter.custom_metric.num_of_response",
        ) {
            Ok(metric_id) => metric_id,
            Err(e) => panic!("Error: {:?}", e),
        };
        true
    }
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(SampleFilter {
            req_metric_id: self.req_metric_id,
            resp_metric_id: self.resp_metric_id,
        }))
    }
}

struct SampleFilter {
    req_metric_id: u32,
    resp_metric_id: u32,
}

impl Context for SampleFilter {}

impl HttpContext for SampleFilter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        proxy_wasm::hostcalls::increment_metric(self.req_metric_id, 1).unwrap();
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        proxy_wasm::hostcalls::increment_metric(self.resp_metric_id, 1).unwrap();
        Action::Continue
    }
}
