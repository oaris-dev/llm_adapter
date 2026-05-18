use llm_adapter::capability::provider_default_capability_upper_bound;

#[test]
fn mistral_models_resolve_capabilities() {
    for model in [
        "mistral-large-latest",
        "mistral-medium-latest",
        "mistral-small-latest",
        "codestral-latest",
    ] {
        let caps = provider_default_capability_upper_bound("mistral", model);
        assert!(caps.is_some(), "no capability upper bound for mistral/{model}");
        println!("  mistral/{model} -> {:?}", caps.unwrap());
    }
    // sanity: an unknown provider must still be rejected
    assert!(provider_default_capability_upper_bound("nope", "x").is_none());
}
