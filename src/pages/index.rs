use leptos::prelude::*;

use crate::components::layout::section::Section;
use crate::components::element::skills_card::SkillsCard;
//use crate::components::element::works_card::WorksCard;
use crate::components::element::skills_card::{Proficiency, Stack};
use crate::components::element::toc::Toc;
use crate::layouts::layout::Layout;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
            <div class="bg-cyber-page bg-cover bg-fixed bg-center h-screen">
                <Layout status=1>
                    <Section title="Table of content" tag="">
                        <Toc title = "About me" tag="#about" />
                        <Toc title = "Skills" tag="#skills" />
                        <Toc title = "Works" tag="#works" />
                    </Section>
                    <Section title="TaiTan291" tag="">
                        <p>"高専2年 知能情報コース所属"</p>
                    </Section>
                    <Section title="About Me" tag="About">
                        <p>"高専2年 知能情報コース所属のtaitanです。"<br/>"現在はwebフロントエンドやRustなどの勉強中です。"</p>
                    </Section>
                    <Section title="Skills" tag="Skills">
                        <div class="flex w-full">
                            <SkillsCard
                                name="Astro"
                                image_url="public/astro_icon.svg"
                                skills=vec![
                                    (Stack::Frontend,Proficiency::Learning),
                                ]
                            />
                            <SkillsCard
                                name="Rust"
                                image_url="public/rust_icon.svg"
                                skills=vec![
                                    (Stack::Application,Proficiency::Learning),
                                    (Stack::Frontend,Proficiency::Learning),
                                ]
                            />
                            <SkillsCard
                                name="Python"
                                image_url="public/python_icon.svg"
                                skills=vec![
                                    (Stack::Application,Proficiency::Learning),
                                ]
                            />
                        </div>
                        <div class="flex w-full">
                            <SkillsCard
                                name="Git"
                                image_url="public/git_icon.svg"
                                skills=vec![
                                    (Stack::Infrastructure,Proficiency::Learning),
                                ]
                            />
                            <SkillsCard
                                name="GitHub"
                                image_url="public/github_icon.svg"
                                skills=vec![
                                    (Stack::Infrastructure,Proficiency::Learning),
                                ]
                            />
                        </div>
                    </Section>
                    /*
                    <Section title="Works">
                        <WorksCard
                            title="うんち"
                            image_url=""
                            stack={vec!["Rust".to_string()]}
                            md_path=""
                        />
                    </Section>
                    */
                </Layout>
            </div>
        }
}
