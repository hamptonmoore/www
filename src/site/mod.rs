use maud::{html, Markup};
use crate::SiteState;
pub mod home;

pub fn base(content: Markup, state: SiteState) -> Markup {
    let last_updated = state.last_updated.clone();
    let build_info = format!("Built on: {} • Ref: {} • Commit: {} • CT: {}",
                             std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")),
                             std::env::var("CT").unwrap_or_else(|_| String::from("Unknown")),
                             );

    html! {
        (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    link rel="stylesheet" href="/assets/css/pure-min.css";
                    link rel="stylesheet" href="/assets/css/main.css";
                    link rel="stylesheet" href="/assets/css/grids-responsive-min.css";

                    title { "Ezri" };
                }
                body {
                    div class="main" {
                        (content);
                        div class="footer" {
                            div class="badges" {
                                img src="/assets/img/badges/ezri.png" alt="Ezri";
                                img src="/assets/img/badges/ezricloud.png" alt="EzriCloud";
                                a href="https://kate.pet" { 
                                    img src="/assets/img/badges/kate.gif" alt="kate.pet";
                                }
                                a href="https://easrng.net" {
                                    img src="/assets/img/badges/easrng.gif" alt="easrng";
                                }
                                a href="https://s-mith.github.io/awfulwebsite/" {
                                    img src="/assets/img/badges/lily.gif" alt="lily";
                                }
                                a href="https://tilde.town/" {
                                    img src="/assets/img/badges/tildetownpink.gif" alt="tilde.town";
                                }
                                img src="/assets/img/badges/xenia-now.gif" alt="xenia-now";
                                img src="/assets/img/badges/vimlove.gif" alt="vim";
                                a href="https://xenyth.net/" {
                                    img src="/assets/img/badges/xenyth.png" alt="xenyth cloud";
                                }
                                img src="/assets/img/badges/aperture_labs.jpg" alt="aperture_labs";
                                img src="/assets/img/badges/nb_noproblem.jpg" alt="nonbinary_noproblem";
                            }

                            p {
                                "Auto refreshed: " (last_updated)
                                    br;
                                    "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                                    br;
                                "Source code "
                                    a target="_blank" href="https://github.com/ezrizhu/www2" { "available here" }
                                ", released under the "
                                    a target="_blank" href="https://github.com/ezrizhu/www2/blob/main/COPYING" { "GNU AGPLv3 license" }
                                "."
                                    br;
                                (build_info);
                                br;
                                "Art by "
                                a href="https://toyhou.se/StandbySnail" { "StandbySnail" } " and " 
                                a href="https://v3ss33l.crd.co/" { "V3SS33L" }"."
                            }
                        }
                    }

                }
            }
    }
}
