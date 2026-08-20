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

#[test]
fn mistral_chat_models_declare_structured_output() {
    // ModelOutput::Object and ModelOutput::Structured are distinct variants.
    // The chat path calls streamObject, whose slot requires Structured; declaring
    // only Object made the router drop every Mistral target and fail the request
    // with managed_preset_unavailable.
    use llm_adapter::capability::ModelOutput;
    for model in [
        "mistral-large-latest",
        "mistral-medium-latest",
        "mistral-small-latest",
        "codestral-latest",
    ] {
        let caps = provider_default_capability_upper_bound("mistral", model)
            .unwrap_or_else(|| panic!("no capabilities for {model}"));
        assert!(
            caps.iter().any(|c| c.output.contains(&ModelOutput::Structured)),
            "{model} must declare structured output"
        );
        assert!(
            caps.iter().any(|c| c.output.contains(&ModelOutput::Text)),
            "{model} must declare text output"
        );
    }
}

#[test]
fn mistral_chat_models_declare_tool_calling() {
    // AFFiNE chat always sends tools (searchWorkspace / readingDocs), so the
    // router adds ModelFeature::ToolCalling to the slot requirements. Without it
    // every Mistral target is filtered out.
    use llm_adapter::capability::ModelFeature;
    for model in ["mistral-large-latest", "mistral-medium-latest", "mistral-small-latest", "codestral-latest"] {
        let caps = provider_default_capability_upper_bound("mistral", model)
            .unwrap_or_else(|| panic!("no capabilities for {model}"));
        assert!(
            caps.iter().any(|c| c.features.contains(&ModelFeature::ToolCalling)),
            "{model} must declare ToolCalling"
        );
    }
}
