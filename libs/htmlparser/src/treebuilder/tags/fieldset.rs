// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// bee-tools-codegen.js --no-escape --input-stdin tag.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn handle_start_fieldset(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            let span = tracing::debug_span!("handle_start_fieldset", mode = ?self.mode);
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
                        // TODO: Set the head element pointer to the newly created head element.
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InHead) => {
                    let ctrl = {
                        debug_assert!(self.context().is_html_element(tag!(Head)));
                        self.pop_element();
                        self.switch_to(mode!(AfterHead));
                        Control::Reprocess
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
                        self.push_html_body_element(&Tag::with_no_attrs("body"));
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InBody, InCaption, InCell) => {
                    let ctrl = {
                        if self.context().has_p_element_in_button_scope() {
                            self.close_p_element();
                        }
                        self.push_html_fieldset_element(tag);
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTable, InTableBody, InRow) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        self.enable_foster_parenting();
                        let ctrl = {
                            if self.context().has_p_element_in_button_scope() {
                                self.close_p_element();
                            }
                            self.push_html_fieldset_element(tag);
                            Control::Continue
                        };
                        self.disable_foster_parenting();
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
                mode!(InColumnGroup) => {
                    let ctrl = {
                        if !self.context().is_html_element(tag!(Colgroup)) {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            // Ignore the token.
                            tracing::debug!("Ignore the token");
                            Control::Continue
                        } else {
                            self.pop_element();
                            self.switch_to(mode!(InTable));
                            Control::Reprocess
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(
                    InSelect,
                    InSelectInTable,
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
                mode!(InTemplate) => {
                    let ctrl = {
                        // TODO: Pop the current template insertion mode off the stack of template insertion modes.
                        // TODO: Push "in body" onto the stack of template insertion modes so that it is the new current template insertion mode.
                        self.switch_to(mode!(InBody));
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
                mode!(Text) => {
                    unreachable!();
                }
            }
        }
    }

    #[allow(unused_variables)]
    pub fn handle_end_fieldset(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            let span = tracing::debug_span!("handle_end_fieldset", mode = ?self.mode);
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
                    InHead,
                    InHeadNoscript,
                    AfterHead,
                    InSelect,
                    InSelectInTable,
                    InTemplate,
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
                mode!(InBody, InCaption, InCell) => {
                    let ctrl = {
                        if !self.context().has_fieldset_element_in_scope() {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            // Ignore the token.
                            tracing::debug!("Ignore the token");
                            Control::Continue
                        } else {
                            self.close_implied_tags();
                            if !self.context().is_html_element(tag!(Fieldset)) {
                                // TODO: Parse error.
                                tracing::debug!("Parse error");
                            }
                            while !self.context().is_html_element(tag!(Fieldset)) {
                                self.pop_element();
                            }
                            self.pop_element(); // pop an html fieldset element
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
                mode!(InTable, InTableBody, InRow) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        self.enable_foster_parenting();
                        let ctrl = {
                            if !self.context().has_fieldset_element_in_scope() {
                                // TODO: Parse error.
                                tracing::debug!("Parse error");
                                // Ignore the token.
                                tracing::debug!("Ignore the token");
                                Control::Continue
                            } else {
                                self.close_implied_tags();
                                if !self.context().is_html_element(tag!(Fieldset)) {
                                    // TODO: Parse error.
                                    tracing::debug!("Parse error");
                                }
                                while !self.context().is_html_element(tag!(Fieldset)) {
                                    self.pop_element();
                                }
                                self.pop_element(); // pop an html fieldset element
                                Control::Continue
                            }
                        };
                        self.disable_foster_parenting();
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
                mode!(InColumnGroup) => {
                    let ctrl = {
                        if !self.context().is_html_element(tag!(Colgroup)) {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            // Ignore the token.
                            tracing::debug!("Ignore the token");
                            Control::Continue
                        } else {
                            self.pop_element();
                            self.switch_to(mode!(InTable));
                            Control::Reprocess
                        }
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
