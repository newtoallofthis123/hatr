# The Hatr

The rust powered HTMX stack we all need.

## What is it?

Hatr stands for
- HTMX
- Actix
- Tailwind
- Rust

You already guessed what it is.
So, all the work is split into two parts:
The frontend powered by HTMX styled with Tailwind and the backend powered by Actix and Rust.
It is very simple and easy to use.

## How to use it?

Well, the current repository is sort of a good starting point.

The current repo has all the html files in the files folder and all the rust server code in the src folder.

The actix server is configured to serve the files from the files folder as well as the css files from the static folder.
The static folder is where you should put all your css files and then use the already included tailwind.config.js file to build your css.

The tailwind.config.js file is configured to watch the html files in the files folder and build the css files in the static folder.
This makes sure that you don't include that playground CDN in your production code.

So, here's a break down

```
hatr/
├── files/
│   ├── index.html
│   └── about.html
├── src/
│   ├── main.rs
│   └── ...
├── static/
│   ├── tailwind.config.js
│   └── ...
├── Cargo.toml
└── ...
```

So, that is the basic structure of the project.
You are free to change it as you like, but this one I think works the best.

## What is this?

Well, the current repository is a simple Todo app built on a sqlite database.
It is a very simple app and is not meant to be used in production.

However, it is a good starting point for your own projects.
So, feel free to check out the code, I didn't code everything, in the coming days I will try and complete this project.

## Contributing

I just had this idea and I am not sure if it is a good one.
So, I would love to hear your thoughts on this.

If you have any suggestions, feel free to open an issue or a pull request.

Moreover, it would be really cool if you could share this project with a fellow rustacean and try to help complete the todo app.

## License

This project is licensed under the MIT License.
You can find the license in the [LICENSE](LICENSE) file.