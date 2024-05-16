#[cfg(test)]
mod tests {
    use crate::lib::core::filio::*;


    #[test]
    fn test_filio_new() {
        let filio: Filio = Filio::new(
            "input".to_string(),
            "output".to_string(),
            "extension".to_string(),
            "action".to_string(),
            "prefix".to_string(),
        );

        assert_eq!(filio.input,"input".to_string());
        assert_eq!(filio.output,"output".to_string());
        assert_eq!(filio.extension,"extension".to_string());
        assert_eq!(filio.action,"action".to_string());
        assert_eq!(filio.prefix,"prefix".to_string());

    }

    //TODO: test filio_listen
}