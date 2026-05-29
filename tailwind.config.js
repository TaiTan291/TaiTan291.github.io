/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
	theme: {
    extend: {
      colors: {
        cyber: {
          page:      "#FFFFFF",
					container: "#EEEDFE",
					layout:    "#F4F3FF",
					surface:   "#534AB7",

					text:      "#0F172A",
					layouttext:"#FFFFFF",

					muted:     "#64748B",
					accent:    "#4F46E5",
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
