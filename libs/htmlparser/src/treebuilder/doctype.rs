// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/htmlparser/src/treebuilder/doctype.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn handle_doctype(&mut self, doctype: Doctype<'_>) -> Control {
        self.ignore_lf = false;
        loop {
            logger::debug!(mode = ?self.mode);
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if let Some(name) = doctype.name {
                            if name != "html" {
                                // TODO: Parse error.
                                logger::debug!("Parse error");
                            }
                        }
                        if doctype.public_id.is_some() {
                            // TODO: Parse error.
                            logger::debug!("Parse error");
                        }
                        if let Some(system_id) = doctype.system_id {
                            if system_id != "about:legacy-compat" {
                                // TODO: Parse error.
                                logger::debug!("Parse error");
                            }
                        }
                        self.append_doctype(&doctype);
                        self.determine_quirks_mode(&doctype);
                        self.switch_to(mode!(BeforeHtml));
                        Control::Continue
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
                    InBody,
                    InTable,
                    InCaption,
                    InColumnGroup,
                    InTableBody,
                    InRow,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    InTemplate,
                    AfterBody,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterBody,
                    AfterAfterFrameset
                ) => {
                    let ctrl = {
                        // TODO: Parse error.
                        logger::debug!("Parse error");
                        // Ignore the token.
                        logger::debug!("Ignore the token");
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
                            logger::debug!("Parse error");
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
                mode!(Text) => {
                    unreachable!();
                }
            }
        }
    }

    fn determine_quirks_mode(&mut self, doctype: &Doctype<'_>) {
        if self.iframe_srcdoc {
            return;
        }
        if !self.quirks_mode_changeable {
            return;
        }
        if doctype.force_quirks {
            self.quirks_mode = QuirksMode::Quirks;
            return;
        }
        if let Some(name) = doctype.name {
            if name != "html" {
                self.quirks_mode = QuirksMode::Quirks;
                return;
            }
        }
        if let Some(public_id) = doctype.public_id {
            // TODO: use a hashmap
            static QUIRKS_PUBLIC_IDS: [&str; 3] = [
                "-//W3O//DTD W3 HTML Strict 3.0//EN//",
                "-/W3C/DTD HTML 4.0 Transitional/EN",
                "HTML",
            ];
            if QUIRKS_PUBLIC_IDS
                .iter()
                .any(|id| public_id.eq_ignore_ascii_case(id))
            {
                self.quirks_mode = QuirksMode::Quirks;
                return;
            }
            // TODO: use a trie
            static QUIRKS_PUBLIC_ID_PREFIXES: [&str; 55] = [
                "+//Silmaril//dtd html Pro v0r11 19970101//",
                "-//AS//DTD HTML 3.0 asWedit + extensions//",
                "-//AdvaSoft Ltd//DTD HTML 3.0 asWedit + extensions//",
                "-//IETF//DTD HTML 2.0 Level 1//",
                "-//IETF//DTD HTML 2.0 Level 2//",
                "-//IETF//DTD HTML 2.0 Strict Level 1//",
                "-//IETF//DTD HTML 2.0 Strict Level 2//",
                "-//IETF//DTD HTML 2.0 Strict//",
                "-//IETF//DTD HTML 2.0//",
                "-//IETF//DTD HTML 2.1E//",
                "-//IETF//DTD HTML 3.0//",
                "-//IETF//DTD HTML 3.2 Final//",
                "-//IETF//DTD HTML 3.2//",
                "-//IETF//DTD HTML 3//",
                "-//IETF//DTD HTML Level 0//",
                "-//IETF//DTD HTML Level 1//",
                "-//IETF//DTD HTML Level 2//",
                "-//IETF//DTD HTML Level 3//",
                "-//IETF//DTD HTML Strict Level 0//",
                "-//IETF//DTD HTML Strict Level 1//",
                "-//IETF//DTD HTML Strict Level 2//",
                "-//IETF//DTD HTML Strict Level 3//",
                "-//IETF//DTD HTML Strict//",
                "-//IETF//DTD HTML//",
                "-//Metrius//DTD Metrius Presentational//",
                "-//Microsoft//DTD Internet Explorer 2.0 HTML Strict//",
                "-//Microsoft//DTD Internet Explorer 2.0 HTML//",
                "-//Microsoft//DTD Internet Explorer 2.0 Tables//",
                "-//Microsoft//DTD Internet Explorer 3.0 HTML Strict//",
                "-//Microsoft//DTD Internet Explorer 3.0 HTML//",
                "-//Microsoft//DTD Internet Explorer 3.0 Tables//",
                "-//Netscape Comm. Corp.//DTD HTML//",
                "-//Netscape Comm. Corp.//DTD Strict HTML//",
                "-//O'Reilly and Associates//DTD HTML 2.0//",
                "-//O'Reilly and Associates//DTD HTML Extended 1.0//",
                "-//O'Reilly and Associates//DTD HTML Extended Relaxed 1.0//",
                "-//SQ//DTD HTML 2.0 HoTMetaL + extensions//",
                "-//SoftQuad Software//DTD HoTMetaL PRO 6.0::19990601::extensions to HTML 4.0//",
                "-//SoftQuad//DTD HoTMetaL PRO 4.0::19971010::extensions to HTML 4.0//",
                "-//Spyglass//DTD HTML 2.0 Extended//",
                "-//Sun Microsystems Corp.//DTD HotJava HTML//",
                "-//Sun Microsystems Corp.//DTD HotJava Strict HTML//",
                "-//W3C//DTD HTML 3 1995-03-24//",
                "-//W3C//DTD HTML 3.2 Draft//",
                "-//W3C//DTD HTML 3.2 Final//",
                "-//W3C//DTD HTML 3.2//",
                "-//W3C//DTD HTML 3.2S Draft//",
                "-//W3C//DTD HTML 4.0 Frameset//",
                "-//W3C//DTD HTML 4.0 Transitional//",
                "-//W3C//DTD HTML Experimental 19960712//",
                "-//W3C//DTD HTML Experimental 970421//",
                "-//W3C//DTD W3 HTML//",
                "-//W3O//DTD W3 HTML 3.0//",
                "-//WebTechs//DTD Mozilla HTML 2.0//",
                "-//WebTechs//DTD Mozilla HTML//",
            ];
            // TODO: ascii case-insensitive
            if QUIRKS_PUBLIC_ID_PREFIXES
                .iter()
                .any(|prefix| public_id.starts_with(prefix))
            {
                self.quirks_mode = QuirksMode::Quirks;
                return;
            }
            if doctype.system_id.is_none() {
                static QUIRKS_PUBLIC_ID_PREFIXES: [&str; 2] = [
                    "-//W3C//DTD HTML 4.01 Frameset//",
                    "-//W3C//DTD HTML 4.01 Transitional//",
                ];
                // TODO: ascii case-insensitive
                if QUIRKS_PUBLIC_ID_PREFIXES
                    .iter()
                    .any(|prefix| public_id.starts_with(prefix))
                {
                    self.quirks_mode = QuirksMode::Quirks;
                    return;
                }
            }
        }
        if let Some(system_id) = doctype.system_id {
            // TODO: use a hashmap
            static QUIRKS_SYSTEM_IDS: [&str; 1] =
                ["http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd"];
            if QUIRKS_SYSTEM_IDS
                .iter()
                .any(|id| system_id.eq_ignore_ascii_case(id))
            {
                self.quirks_mode = QuirksMode::Quirks;
                return;
            }
        }
        if let Some(public_id) = doctype.public_id {
            // TODO: use a trie
            static LIMITED_QUIRKS_PUBLIC_IDS: [&str; 2] = [
                "-//W3C//DTD XHTML 1.0 Frameset//",
                "-//W3C//DTD XHTML 1.0 Transitional//",
            ];
            if LIMITED_QUIRKS_PUBLIC_IDS
                .iter()
                .any(|id| public_id.eq_ignore_ascii_case(id))
            {
                self.quirks_mode = QuirksMode::LimitedQuirks;
                return;
            }
            if doctype.system_id.is_some() {
                // TODO: duplicated, use a trie
                static QUIRKS_PUBLIC_ID_PREFIXES: [&str; 2] = [
                    "-//W3C//DTD HTML 4.01 Frameset//",
                    "-//W3C//DTD HTML 4.01 Transitional//",
                ];
                // TODO: ascii case-insensitive
                if QUIRKS_PUBLIC_ID_PREFIXES
                    .iter()
                    .any(|prefix| public_id.starts_with(prefix))
                {
                    self.quirks_mode = QuirksMode::Quirks;
                }
            }
        }
    }
}
