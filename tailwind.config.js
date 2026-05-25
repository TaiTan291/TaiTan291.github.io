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
				stack: {
					frontend: {
						bg:     "#EEEDFE",
						text:   "#534AB7",
						bar:    "#534AB7",
						border: "#AFA9EC",
					},

					backend: {
						bg:      "#E6F1FB",
						text:    "#185FA5",
						bar:     "#185FA5",
						border:  "#85B7EB",
					},

					app: {
						bg:          "#E1F5EE",
						text:        "#0F6E56",
						bar:         "#1D9E75",
						border:      "#5DCAA5",
					},

					infra: {
						bg:        "#FAEEDA",
						text:      "#854F0B",
						bar:       "#EF9F27",
						border:    "#EF9F27",
					},
				},
      },
		},
  },
  plugins: [],
}
