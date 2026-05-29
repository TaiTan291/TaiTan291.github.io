use leptos::prelude::*;
use leptos_router::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use leptos::web_sys;

use crate::components::layout::section::Section;
use crate::components::element::toc::{LinkToc, GorioshiLinkToc};
use crate::components::element::item::{OshiBlock, OshiItem};
use crate::components::element::dropdown::Dropdown;
use crate::layouts::layout::Layout;

const ACCESS_KEY: &str = "favorite_access_granted";
const KEY_VALUE: &str = "true";

#[component]
pub fn FavoritePage() -> impl IntoView {
    view! {
        <Layout status=2>
            <Section title="About" tag="#about">
                <div class="flex flex-col md:flex-row items-center gap-6 p-2">
                <div class="w-24 h-24 rounded-full bg-cyber-accent/20 border border-cyber-accent flex items-center justify-center shrink-0">
                    <img
                        src="public/my_icon.avif"
                        alt="My icon"
                        class="w-20 h-20 m-1 rounded-full object-cover"
                    />
                </div>

                <dl class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-base md:text-lg">
                    <dt class="font-bold text-cyber-accent">"名前："</dt>
                    <dd class="text-cyber-text">"たいたん"</dd>

                    <dt class="font-bold text-cyber-accent">"誕生日："</dt>
                    <dd class="text-cyber-text">"8月11日"</dd>

                    <dt class="font-bold text-cyber-accent">"出身："</dt>
                    <dd class="text-cyber-text">"日本/関西"</dd>
                </dl>
            </div>
            </Section>
            <Section title="趣味全般" tag="#hobby">
                <div class="flex flex-col">
                    <Dropdown title="星空観察">
                        <p>{"星を見るのはいいぞ〜"}</p>
                    </Dropdown>
                    <Dropdown title="アニメ/アニメ映画を見る">
                        <p>"昔のアニメはあんま見ない(長編のやつも)、きまま~に見る"</p>
                    </Dropdown>
                    <Dropdown title="linuxのdotfile(設定)をいじる">
                        <p>
                            "私は独自系のlinuxディストリビューションであるnixosを現在のメイン機で使用している(ちなみにその前は同じく独自系のarch linuxを使用していた)。基本的にそこまでデザインにこだわることはそこまでないが、なぜか永遠にいじりたいことがあって終らない。ちなみに私のgithubのpublicにあるpushしている。("
                            <a 
                                href="https://github.com/TaiTan291/dotfiles"
                                target="_blank" 
                                class="text-blue-500 underline hover:text-blue-700"
                            >
                                "こ↑こ↓"
                            </a>
                            ")ただ、カスみたいなcommitやコードなので参考にするのはやめておいた方がいい"
                        </p>
                    </Dropdown>
                </div>
            </Section>
            <Section title="Vtuber/Youtuber" tag="#vtuber">
                <OshiBlock>
                    <OshiItem label="最推し">"ういだがー"</OshiItem>
                    <OshiItem label="推し歴">"もうすぐ5ヶ月"</OshiItem>
                    <OshiItem label="推しはじめた理由">"ういままの配信に突然現れたときに一目惚れした"</OshiItem>
                    <OshiItem label="最推しの推しポイント">
                        "なんといってもういままにデザインされたクリクリとし大きいおめめ!ぬいとの相性も良くてぬいもういだがーどちらも引きたさせる\n他にもフリップ芸やUitopiaでのファンサなどすべての動きがかわいい。"
                    </OshiItem>
                </OshiBlock>
                <OshiBlock>
                    <OshiItem label="推し">"しぐれうい"</OshiItem>
                    <OshiItem label="推し歴">"多分2年くらい？"</OshiItem>
                    <OshiItem label="推しはじめた理由">
                        "元から切り抜き等で知っていてロリ神をきっかけにリアタイで配信を見るようになった。\nそしてゴミへと昇華した。"
                    </OshiItem>
                    <OshiItem label="ui">
                        "なんといっても可愛い！でも、可愛いだけじゃなくて、ゲームが弱かったり体力がなかったりする一見ポンコツな一面も愛おしいです。オタクに対して口調が強く見えて、実はめちゃくちゃ優しいところも最高。\n\nイラストレーターやVtuber、そして本業（？）のアイマスPを両立しているのは本当に凄い。ライブのためにジムに通ったり、朝4時からランニングしたりする努力家な姿も心から尊敬できるところ！"
                    </OshiItem>
                </OshiBlock>
                <OshiBlock>
                    <OshiItem label="最近見てる/気になってる人①">"ぶいすぽ"</OshiItem>
                    <OshiItem label="特に……">"なずぴやサイネちゃんを見ている"</OshiItem>
                    <OshiItem label="好きなところ">"いろんなストリーマーさんとからんでいたりespoertに全力でとりくんでいるところがいい"</OshiItem>
                </OshiBlock>
                <OshiBlock>
                    <OshiItem label="最近見てる/気になってる人②">"GON"</OshiItem>
                    <OshiItem label="見はじめた理由">"vspo showdownでコーチをしていて知った。見ていてあきない面白さがある。\n基本Youtubeの切り抜きでしかみないが配信にもいってみたい"</OshiItem>
                </OshiBlock>
                <OshiBlock>
                    <OshiItem label="最近見てる/気になってる人③">"犬山たまきくん"</OshiItem>
                    <OshiItem label="見る頻度">"切り抜き動画を良くみる、たまにリアタイしたりアーカイブも見にいく"</OshiItem>
                    <OshiItem label="好きなところ">"なんといってもめあたむ回やぶったぎりる配信がおもしろい"</OshiItem>
                </OshiBlock>
                <OshiBlock>
                    <OshiItem label="最近見てる/気になってる人④">"めめ村(Among Usなどのゆっくり実況)"</OshiItem>
                    <OshiItem label="">"なんやかんやでずっとめめ村の住民の動画をずっと見ている"</OshiItem>
                </OshiBlock>
            </Section>
            <Section title="Link" tag="#link">
                <GorioshiLinkToc title="Twitter" url="https://twitter.com/Taitan291" image_url="public/x_icon.svg" />
                <LinkToc title="github" url="https://github.com/TaiTan291" image_url="public/github_icon.svg" />
            </Section>
        </Layout>
    }
}
#[component]
pub fn Favorite() -> impl IntoView {
    let navigate = use_navigate();
    let (authorized, set_authorized) = signal(Option::<bool>::None);

    Effect::new(move |_| {
        let is_valid = check_and_consume_key();
        set_authorized.set(Some(is_valid));

        if !is_valid {
            navigate("/404", NavigateOptions {
                replace: true,
                ..Default::default()
            });
        }
    });

    view! {
        <Show
            when=move || authorized.get() == Some(true)
            fallback=|| view! { <div></div> }
        >
            <FavoritePage/>
        </Show>
    }
}




#[component]
pub fn Gateway() -> impl IntoView {
    let navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(storage) = get_session_storage() {
            let _ = storage.set_item(ACCESS_KEY, KEY_VALUE);
        }
        navigate("/hobby", NavigateOptions {
            replace: true,
            ..Default::default()
        });
    });

    view! {
        <div>"Redirecting..."</div>
    }
}


fn get_session_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.session_storage().ok()?
}

fn check_and_consume_key() -> bool {
    let storage = match web_sys::window()
        .and_then(|w| w.session_storage().ok())
        .flatten()
    {
        Some(s) => s,
        None => return false,
    };

    match storage.get_item(ACCESS_KEY) {
        Ok(Some(_)) => {
            let _ = storage.remove_item(ACCESS_KEY);
            true
        }
        _ => false,
    }
}
