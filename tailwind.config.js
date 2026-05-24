/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
	theme: {
    extend: {
      colors: {
        cyber: {
          // 背景・ベース
          page:      "##e2e8f0",  // ページ背景
          container: "#020617",  // コンテナ / Section
          surface:   "#0f172a",  // Header / Footer
					block:     "#242426",  // block
					layout:    "#0d2137",

          // アクセント
          accent:    "#FFFFFF",  // メインアクセント（見出し・ボーダー・タグ）
          link:      "#4499ff",  // サブアクセント（リンク・ホバー）

          // テキスト
          text:      "#FFFFFF",  // メインテキスト
          muted:     "#FFFFFF",  // サブテキスト
        },
      },
    },
  },
  plugins: [],
}
