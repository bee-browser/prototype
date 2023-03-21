// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// bee-tools-codegen.js --no-escape --input-stdin tag.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn handle_start_dt(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            let span = tracing::debug_span!("handle_start_dt", mode = ?self.mode);
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
                        self.frameset_ok = false;
                        let mut context_pos = self.context_stack.len() - 1;
                        loop {
                            let context = &self.context_stack[context_pos];
                            if context.is_html_element(tag!(Dd)) {
                                self.close_implied_tags_except_for(tag!(Dd));
                                if !self.context().is_html_element(tag!(Dd)) {
                                    // TODO: Parse error.
                                    tracing::debug!("Parse error");
                                }
                                while !self.context().is_html_element(tag!(Dd)) {
                                    self.pop_element();
                                }
                                self.pop_element(); // pop an html dd element
                                break;
                            }
                            if context.is_html_element(tag!(Dt)) {
                                self.close_implied_tags_except_for(tag!(Dt));
                                if !self.context().is_html_element(tag!(Dt)) {
                                    // TODO: Parse error.
                                    tracing::debug!("Parse error");
                                }
                                while !self.context().is_html_element(tag!(Dt)) {
                                    self.pop_element();
                                }
                                self.pop_element(); // pop an html dt element
                                break;
                            }
                            let local_name = context.local_name();
                            if local_name.is_special() {
                                if context.is_html()
                                    && !tags![Address, Div, P].contains(&local_name)
                                {
                                    break;
                                }
                            }
                            context_pos -= 1;
                        }
                        if self.context().has_p_element_in_button_scope() {
                            self.close_p_element();
                        }
                        self.push_html_dt_element(tag);
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
                            self.frameset_ok = false;
                            let mut context_pos = self.context_stack.len() - 1;
                            loop {
                                let context = &self.context_stack[context_pos];
                                if context.is_html_element(tag!(Dd)) {
                                    self.close_implied_tags_except_for(tag!(Dd));
                                    if !self.context().is_html_element(tag!(Dd)) {
                                        // TODO: Parse error.
                                        tracing::debug!("Parse error");
                                    }
                                    while !self.context().is_html_element(tag!(Dd)) {
                                        self.pop_element();
                                    }
                                    self.pop_element(); // pop an html dd element
                                    break;
                                }
                                if context.is_html_element(tag!(Dt)) {
                                    self.close_implied_tags_except_for(tag!(Dt));
                                    if !self.context().is_html_element(tag!(Dt)) {
                                        // TODO: Parse error.
                                        tracing::debug!("Parse error");
                                    }
                                    while !self.context().is_html_element(tag!(Dt)) {
                                        self.pop_element();
                                    }
                                    self.pop_element(); // pop an html dt element
                                    break;
                                }
                                let local_name = context.local_name();
                                if local_name.is_special() {
                                    if context.is_html()
                                        && !tags![Address, Div, P].contains(&local_name)
                                    {
                                        break;
                                    }
                                }
                                context_pos -= 1;
                            }
                            if self.context().has_p_element_in_button_scope() {
                                self.close_p_element();
                            }
                            self.push_html_dt_element(tag);
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
                        // TODO
                        if !self.text.is_empty() {
                            self.insert_text_to_foster_parent();
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
    pub fn handle_end_dt(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            let span = tracing::debug_span!("handle_end_dt", mode = ?self.mode);
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
                        if !self.context().has_dt_element_in_scope() {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            // Ignore the token.
                        } else {
                            self.close_implied_tags_except_for(tag!(Dt));
                            if !self.context().is_html_element(tag!(Dt)) {
                                // TODO: Parse error.
                                tracing::debug!("Parse error");
                            }
                            while !self.context().is_html_element(tag!(Dt)) {
                                self.pop_element();
                            }
                            self.pop_element(); // pop an html dt element
                        }
                        Control::Continue
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
                            if !self.context().has_dt_element_in_scope() {
                                // TODO: Parse error.
                                tracing::debug!("Parse error");
                                // Ignore the token.
                            } else {
                                self.close_implied_tags_except_for(tag!(Dt));
                                if !self.context().is_html_element(tag!(Dt)) {
                                    // TODO: Parse error.
                                    tracing::debug!("Parse error");
                                }
                                while !self.context().is_html_element(tag!(Dt)) {
                                    self.pop_element();
                                }
                                self.pop_element(); // pop an html dt element
                            }
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
                        // TODO
                        if !self.text.is_empty() {
                            self.insert_text_to_foster_parent();
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
