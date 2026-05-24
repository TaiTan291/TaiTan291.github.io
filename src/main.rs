use crate::components::element::skills_card::SkillsCard;
use crate::components::layout::section::Section;
//use crate::components::element::works_card::WorksCard;
use crate::components::element::skills_card::{Proficiency, Stack};
use crate::layouts::main_layout::MainLayouts;
use leptos::prelude::*;
use leptos::*;

//pub mod app;
pub mod components;
pub mod layouts;
//pub mod views;

fn main() {
    // パニック時にブラウザコンソールにエラーを出力
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div class="bg-cyber-page bg-cover bg-fixed bg-center h-screen">
                <MainLayouts>
                    <Section title="TaiTan291">
                        <p>"高専2年 知能情報コース所属"</p>
                    </Section>
                    <Section title="About Me">
                        <p>"高専2年 知能情報コース所属のtaitanです。"<br/>"現在はwebフロントエンドやRustなどの勉強中です。"</p>
                    </Section>
                    <Section title="Skills">
                        <div class="flex w-full">
                            <SkillsCard
                                name="Rust"
                                image_url="public/rust_icon.svg"
                                skills=vec![
                                    (Stack::Frontend,Proficiency::Learning),
                                    (Stack::Application,Proficiency::Learning)
                                ]
                            />
                            <SkillsCard
                                name="Astro"
                                image_url=""
                                skills=vec![
                                    (Stack::Frontend,Proficiency::Learning),
                                ]
                            />
                            <SkillsCard
                                name="Python"
                                image_url=""
                                skills=vec![
                                    (Stack::Application,Proficiency::Learning),
                                ]
                            />
                        </div>
                        <div class="flex w-full">
                            <SkillsCard
                                name="Git"
                                image_url=""
                                skills=vec![
                                    (Stack::Infrastructure,Proficiency::Learning),
                                ]
                            />
                            <SkillsCard
                                name="GitHub"
                                image_url=""
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
                </MainLayouts>
            </div>
        }
    })
}
