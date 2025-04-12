#!/bin/bash

# 1. Compiler le CSS Tailwind sans 'watch'
npx tailwindcss -i ./style/tailwind.css -o ./public/tailwind.css

# 2. Démarrer Trunk
trunk serve --open