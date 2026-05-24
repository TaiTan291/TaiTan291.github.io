use leptos::control_flow::Show;
use leptos::prelude::create_signal;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::OnAttribute;
use leptos::prelude::Set;
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
    // proficiency は名前とプログレスバー用の数値の両方が必要なので
    // trait とは別に専用メソッドを定義する
    pub fn label_with_level(&self) -> (&'static str, i8) {
        match self {
            Proficiency::Advanced => ("Advanced", 100),
            Proficiency::Intermediate => ("Intermediate", 70), // 修正: INtermediate -> Intermediate
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

    let skill_entries: Vec<(&'static str, &'static str, i8)> = skills
        .iter()
        .map(|(stack, prof)| {
            let (prof_label, level) = prof.label_with_level();
            (stack.label(), prof_label, level)
        })
        .collect();

    let stack_labels: Vec<String> = skills
        .iter()
        .map(|(stack, _)| stack.label().to_string())
        .collect();

    view! {
        <div
            class="min-w-30 h-40 m-3 flex flex-1 flex-col items-center justify-start bg-cyber-block rounded-md overflow-hidden shadow-sm cursor-pointer hover:brightness-125 transition-all"
            on:click=move |_| set_show_modal.set(true)
        >
            <img
                src={image_url.clone()}
                alt="Logo"
                class="w-24 h-24 mt-2"
            />
            <h2 class="my-4 text-xl text-cyber-text font-bold">{name.clone()}</h2>
        </div>

        <Show when=move || show_modal.get()>
            <div
                class="fixed inset-0 backdrop-blur-sm flex items-center justify-center z-50"
                on:click=move |_| set_show_modal.set(false)
            >
                <div
                    class="bg-cyber-block rounded-xl shadow-2xl w-full max-w-3xl mx-5 h-auto"
                    on:click=|e| e.stop_propagation()
                >
                    <div class="flex items-start p-4 gap-4">
                        <img
                            src={image_url.clone()}
                            alt="Logo"
                            class="w-40 h-40 shrink-0"
                        />
                        <div class="flex flex-col flex-1 min-w-0">
                            <h2 class="text-6xl text-cyber-text font-bold underline decoration-2 underline-offset-4">
                                {name.clone()}
                            </h2>
                            {skill_entries.clone().into_iter().map(|(stack_label, prof_label, level)| {
                                view! {
                                    <div class="grid items-center grid-cols-[0.7fr_0.25fr_0.5fr_2fr] gap-2 w-full">
                                        <span class="rounded text-xs text-cyber-accent px-2 py-0.5 shrink-0">
                                            {stack_label}
                                        </span>
                                        <span>"："</span>
                                        <span class="text-cyber-muted text-sm w-24 shrink-0">{prof_label}</span>
                                        <progress max="100" value={level.to_string()} class="flex-1" />
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                        <button
                            class="text-cyber-muted hover:text-cyber-text text-xl leading-none shrink-0"
                            on:click=move |_| set_show_modal.set(false)
                        >
                            "✕"
                        </button>
                    </div>
                    <div class="flex flex-wrap gap-2 px-4 pb-4 justify-end">
                        {stack_labels.clone().into_iter().map(|tag| {
                            view! {
                                <span class="bg-cyber-surface rounded text-sm text-cyber-accent border border-cyber-accent px-2 py-1">
                                    {tag}
                                </span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </Show>
    }
}
