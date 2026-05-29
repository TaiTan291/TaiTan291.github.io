use leptos::control_flow::Show;
use leptos::prelude::create_signal;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::OnAttribute;
use leptos::prelude::Set;
use leptos::prelude::StyleAttribute;
use leptos::*;

pub trait ChangeLabel {
    fn label(&self) -> &'static str;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Proficiency {
    Advanced,     // 上級者
    Intermediate, // 中級者
    Learning,     // 学習中
    Beginner,     // 初心者
}

impl Proficiency {
    pub fn label_with_level(&self) -> (&'static str, i8) {
        match self {
            Proficiency::Advanced => ("Advanced", 100),
            Proficiency::Intermediate => ("Intermediate", 70),
            Proficiency::Learning => ("Learning", 40),
            Proficiency::Beginner => ("Beginner", 10),
        }
    }
}

impl ChangeLabel for Proficiency {
    fn label(&self) -> &'static str {
        self.label_with_level().0
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Stack {
    Frontend,
    Backend,
    Application,
    Infrastructure,
}

impl Stack {
    pub fn bg_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "bg-stack-frontend-bg",
            Stack::Backend => "bg-stack-backend-bg",
            Stack::Application => "bg-stack-app-bg",
            Stack::Infrastructure => "bg-stack-infra-bg",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "text-stack-frontend-text",
            Stack::Backend => "text-stack-backend-text",
            Stack::Application => "text-stack-app-text",
            Stack::Infrastructure => "text-stack-infra-text",
        }
    }

    pub fn bar_hex(&self) -> &'static str {
        match self {
            Stack::Frontend => "#534AB7",
            Stack::Backend => "#185FA5",
            Stack::Application => "#1D9E75",
            Stack::Infrastructure => "#EF9F27",
        }
    }

    pub fn border_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "border-stack-frontend-border",
            Stack::Backend => "border-stack-backend-border",
            Stack::Application => "border-stack-app-border",
            Stack::Infrastructure => "border-stack-infra-border",
        }
    }

    /// カード代表色: スタックの種類が複数ある場合、最初のものを採用
    pub fn dominant(skills: &[(Stack, Proficiency)]) -> Stack {
        skills.first().map(|(s, _)| *s).unwrap_or(Stack::Backend)
    }
}

impl ChangeLabel for Stack {
    fn label(&self) -> &'static str {
        match self {
            Stack::Frontend => "Frontend",
            Stack::Backend => "Backend",
            Stack::Application => "Application",
            Stack::Infrastructure => "Infrastructure",
        }
    }
}

#[component]
pub fn SkillsCard(
    #[prop(into)] name: String,
    #[prop(into)] image_url: String,
    skills: Vec<(Stack, Proficiency)>,
) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    let dominant = Stack::dominant(&skills);

    let skill_entries: Vec<(Stack, &'static str, i8)> = skills
        .iter()
        .map(|(stack, prof)| {
            let (prof_label, level) = prof.label_with_level();
            (*stack, prof_label, level)
        })
        .collect();

    let stack_labels: Vec<Stack> = skills.iter().map(|(stack, _)| *stack).collect();

    view! {
        <div
            class=format!("min-w-30 h-40 m-2 flex flex-1 flex-col items-center justify-start bg-white border-3 border-yellow-300 rounded-xl overflow-hidden shadow-sm cursor-pointer hover:shadow-md transition-all",
            )
            on:click=move |_| set_show_modal.set(true)
        >
            <div class=format!(
                "mt-2 rounded-xl flex items-center justify-center {}",
                dominant.bg_class()
            )>
                <img
                    src={image_url.clone()}
                    alt="Logo"
                    class="w-20 h-20 m-1 object-contain"
                />
            </div>
            <h2 class="my-4 text-xl text-gray-800 font-medium">{name.clone()}</h2>
        </div>

        <Show when=move || show_modal.get()>
            <div
                class="fixed inset-0 backdrop-blur-sm bg-black/20 flex items-center justify-center z-50"
                on:click=move |_| set_show_modal.set(false)
            >
                <div
                    class="bg-white rounded-2xl shadow-xl w-full max-w-3xl mx-5 h-auto border border-gray-100"
                    on:click=|e| e.stop_propagation()
                >
                    <div class="flex items-start p-5 gap-5">
                        <div class=format!(
                            "rounded-xl flex items-center justify-center shrink-0 {}",
                            dominant.bg_class()
                        )>
                            <img
                                src={image_url.clone()}
                                alt="Logo"
                                class="w-40 h-40 m-2 object-contain"
                            />
                        </div>

                        <div class="flex flex-col flex-1 min-w-0">
                            <h2 class=format!(
                                "text-4xl font-medium underline decoration-2 underline-offset-4 mb-3 {}",
                                dominant.text_class()
                            )>
                                {name.clone()}
                            </h2>

                            {skill_entries.iter().map(|(stack, prof_label, level)| {
                                let bar_style = format!("width: {}%", level);
                                let bar_hex   = stack.bar_hex();
                                view! {
                                    <div class="grid items-center grid-cols-[0.7fr_0.25fr_0.5fr_2fr] gap-2 w-full">
                                        <span class=format!(
                                            "rounded text-xs px-2 py-0.5 shrink-0 {} {}",
                                            stack.bg_class(), stack.text_class()
                                        )>
                                            {stack.label()}
                                        </span>
                                        <span>"："</span>
                                        <span class=format!(
                                            "text-sm w-24 shrink-0 {}",
                                            stack.text_class()
                                        )>
                                            {*prof_label}
                                        </span>
                                        <div class="flex-1 bg-gray-100 rounded-full h-1.5 overflow-hidden">
                                            <div
                                                style=format!("width: {}%; background-color: {};", level, bar_hex)
                                                class="h-full rounded-full"
                                            />
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        <button
                            class="text-gray-400 hover:text-gray-700 text-xl leading-none shrink-0 transition-colors"
                            on:click=move |_| set_show_modal.set(false)
                        >
                            "✕"
                        </button>
                    </div>

                    <div class="flex flex-wrap gap-2 px-5 pb-5 justify-end">
                        {stack_labels.iter().map(|stack| {
                            view! {
                                <span class=format!(
                                    "rounded text-sm px-2 py-1 border {} {} {}",
                                    stack.bg_class(), stack.text_class(), stack.border_class()
                                )>
                                    {stack.label()}
                                </span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </Show>
    }
}
