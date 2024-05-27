tailwind:
    cargo watch -s 'pnpm dlx tailwindcss -i style/tailwind.css -o assets/main.css --watch'

dev:
    cargo watch -x run
