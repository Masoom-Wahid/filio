#[cfg(test)]
mod tests {
    use crate::core::filio::*;


    #[test]
    fn test_filio_new() {
        let filio: Filio = Filio::new(
            "input".to_string(),
            "output".to_string(),
            "html,xml,pdf".to_string(),
            "action".to_string(),
            "prefix".to_string(),
            "main".to_string()
        );

        assert_eq!(filio.input,"input".to_string());
        assert_eq!(filio.output,"output".to_string());
        assert_eq!(filio.extensions,vec![
            "html".to_string(),
            "xml".to_string(),
            "pdf".to_string()
        ]);
        assert_eq!(filio.action,"action".to_string());
        assert_eq!(filio.prefix,"prefix".to_string());
        assert_eq!(filio.names,vec![
            "main".to_string(),
        ]);
    }

    //TODO: test filio_listen
}