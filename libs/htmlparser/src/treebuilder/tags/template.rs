// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// bee-tools-codegen --no-escape --input-stdin tag.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn handle_start_template(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            let span = tracing::debug_span!("handle_start_template", mode = ?self.mode);
            let _enter = span.enter();
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if !self.iframe_srcdoc {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                        }
                        self.change_quirks_mode_if_changeable(QuirksMode::Quirks);
                        self.switch_to(mode!(BeforeHtml));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(BeforeHtml) => {
                    let ctrl = {
                        //debug_assert!(self.writer.is_empty());
                        self.push_html_html_element(&Tag::with_no_attrs("html"));
                        self.switch_to(mode!(BeforeHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(BeforeHead) => {
                    let ctrl = {
                        self.push_html_head_element(&Tag::with_no_attrs("head"));
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(
                    InHead,
                    InBody,
                    InTable,
                    InCaption,
                    InColumnGroup,
                    InTableBody,
                    InRow,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    InTemplate
                ) => {
                    let ctrl = {
                        self.push_html_template_element(tag);
                        self.push_marker_to_active_formatting_contexts();
                        self.context_mut().flags |= flags!(HasTemplateElement);
                        self.frameset_ok = false;
                        self.switch_to(mode!(InTemplate));
                        self.push_template_mode(mode!(InTemplate));
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InHeadNoscript) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        debug_assert!(self.context().is_html_element(tag!(Noscript)));
                        self.pop_element();
                        debug_assert!(self.context().is_html_element(tag!(Head)));
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterHead) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        self.reopen_head_element();
                        let ctrl = {
                            self.push_html_template_element(tag);
                            self.push_marker_to_active_formatting_contexts();
                            self.context_mut().flags |= flags!(HasTemplateElement);
                            self.frameset_ok = false;
                            self.switch_to(mode!(InTemplate));
                            self.push_template_mode(mode!(InTemplate));
                            Control::Continue
                        };
                        self.close_head_element();
                        ctrl
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        if self.pending_table_text_contains_non_whitespace {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            self.enable_foster_parenting();
                            self.reconstruct_active_formatting_elements();
                            let node = self.inner.create_text(self.pending_table_text.as_str());
                            self.insert_node(node);
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                            self.disable_foster_parenting();
                        } else {
                            let node = self.inner.create_text(self.pending_table_text.as_str());
                            self.insert_node(node);
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                        }
                        self.switch_to_original_mode();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterBody, AfterAfterBody) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InFrameset, AfterFrameset, AfterAfterFrameset) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        // Ignore the token.
                        tracing::debug!("Ignore the token");
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(Text) => {
                    unreachable!();
                }
            }
        }
    }

    #[allow(unused_variables)]
    pub fn handle_end_template(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            let span = tracing::debug_span!("handle_end_template", mode = ?self.mode);
            let _enter = span.enter();
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if !self.iframe_srcdoc {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                        }
                        self.change_quirks_mode_if_changeable(QuirksMode::Quirks);
                        self.switch_to(mode!(BeforeHtml));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(
                    BeforeHtml,
                    BeforeHead,
                    InHeadNoscript,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterFrameset
                ) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        // Ignore the token.
                        tracing::debug!("Ignore the token");
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(
                    InHead,
                    AfterHead,
                    InBody,
                    InTable,
                    InCaption,
                    InColumnGroup,
                    InTableBody,
                    InRow,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    InTemplate
                ) => {
                    let ctrl = {
                        if !self.context().has_template_element() {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            // Ignore the token.
                            tracing::debug!("Ignore the token");
                            Control::Continue
                        } else {
                            self.close_all_implied_tags();
                            if !self.context().is_html_element(tag!(Template)) {
                                // TODO: Parse error.
                                tracing::debug!("Parse error");
                            }
                            while !self.context().is_html_element(tag!(Template)) {
                                self.pop_element();
                            }
                            self.pop_element(); // pop an html template element
                            self.active_formatting_element_list
                                .clear_up_to_last_marker();
                            self.pop_template_mode();
                            self.reset_insertion_mode_appropriately();
                            Control::Continue
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(Text) => {
                    let ctrl = {
                        self.pop_element();
                        self.switch_to_original_mode();
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        if self.pending_table_text_contains_non_whitespace {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            self.enable_foster_parenting();
                            self.reconstruct_active_formatting_elements();
                            let node = self.inner.create_text(self.pending_table_text.as_str());
                            self.insert_node(node);
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                            self.disable_foster_parenting();
                        } else {
                            let node = self.inner.create_text(self.pending_table_text.as_str());
                            self.insert_node(node);
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                        }
                        self.switch_to_original_mode();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterBody, AfterAfterBody) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
            }
        }
    }
}
