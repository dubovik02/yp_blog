const TOKEN_LOCAL_STORAGE_KEY = "blog_token";
const UNVISIBLE_CLASS_NAME = "is_unvisible";

const REGISTER_FORM_NAME = "register";
const LOGIN_FORM_NAME = "login";
const POSTS_FORM_NAME = "posts";

const ID_SALT = "_dqwwdq123dddsadssdsad";

let reg_form = document.getElementById("register_form");
let login_form = document.getElementById("login_form");
let posts_form = document.getElementById("posts_form");

let reg_link = document.getElementById("link_to_reg");
let login_link = document.getElementById("link_to_login");

let button_login = document.getElementById("button_login");
let button_reg = document.getElementById("button_reg");

let menu_main = document.getElementById("main_menu");
let menu_login = document.getElementById("login_menu");
let menu_reg = document.getElementById("register_menu");

let menu_new_post = document.getElementById("new_post");

let preloader = document.getElementById("preloader");

let token = localStorage.getItem(TOKEN_LOCAL_STORAGE_KEY);

let currentUser = {};


function setLinksBetweenAuthForms() {

	reg_link.addEventListener("click", () => {
		navigate(REGISTER_FORM_NAME)
	});

	login_link.addEventListener("click", () => {
		navigate(LOGIN_FORM_NAME);
	});
}

function setActionListeners() {
	reg_form.addEventListener("submit", (event) => {
		event.preventDefault();
		setAwaitingUI(true);
		UserRegister()
		.then((res) => {
			currentUser = res.user;
			event.target.reset();
			navigate(POSTS_FORM_NAME);
			renderPosts()
			.then((posts) => {
				setAwaitingUI(false);
			})
			.catch((e) => {
				setAwaitingUI(false);
				alert(e);	
			})
		})
		.catch((e) => {
			setAwaitingUI(false);
			alert(e);
		});
	});

	login_form.addEventListener("submit", (event) => {
		event.preventDefault();
		setAwaitingUI(true);
		UserLogin()
		.then((res) => {
			currentUser = res.user;
			event.target.reset();
			navigate(POSTS_FORM_NAME);
			renderPosts()
			.then((posts) => {
				setAwaitingUI(false);
			})
			.catch((e) => {
				setAwaitingUI(false);
				alert(e);	
			})
		})
		.catch((e) => {
			setAwaitingUI(false);
			alert(e);
		})
	})

	menu_main.addEventListener("click", (event) => {
		event.preventDefault();
		navigate(POSTS_FORM_NAME);
	});

	menu_login.addEventListener("click", (event) => {
		event.preventDefault();
		navigate(LOGIN_FORM_NAME);
	});

	menu_reg.addEventListener("click", (event) => {
		event.preventDefault();
		navigate(REGISTER_FORM_NAME);
	});

	menu_new_post.addEventListener("click", (event) => {
		event.preventDefault();
		if (!currentUser.id) {
			navigate(LOGIN_FORM_NAME); return
		};
		const title = prompt("Введите заголовок поста:");
		if (!title) return;
		const content = prompt("Введите содержание поста:");
		if (!content) return;
		setAwaitingUI(true);
		newPost(title, content)
		.then((res) => {
			renderPosts()
			.then((posts) => {
				setAwaitingUI(false);
			})
			.catch((e) => {
				setAwaitingUI(false);
				alert(e);	
			})
		})
		.catch((e) => {
			setAwaitingUI(false);
			alert(e);
		})		
	});
}

async function renderPosts() {
	setAwaitingUI(true);
	if (!currentUser.id) {
		await tryToGetCurrentUser();
	}
	let container = document.getElementById("posts_container");
	container.innerHTML = '';
	loadPosts()
	.then((res) => {
		let postHtml = ``;
		if (res.posts.lenght == 0) {
			postHtml = `<p class="post__props">К сожалению посты отсутствуют :(</p>`
			container.innerHTML = postHtml;
		}
		else {
			res.posts.forEach((p) => {
				const pid = p.id + ID_SALT;
				const pid2 = p.id + ID_SALT + ID_SALT;
				container.insertAdjacentHTML("beforeend", renderPost(pid, pid2, p.title, p.author_id, p.content));
				let elemUDelete = document.getElementById(pid);
				let elemUpdate = document.getElementById(pid2);
				if (p.author_id == currentUser.id) {
					elemUDelete.addEventListener("click", () => {
						const result = confirm("Удалить пост?");
						setAwaitingUI(true);
						if(result===true) {
							delPost(BigInt(p.id))
							.then((res) => {
								renderPosts()
								.then((posts) => {
									setAwaitingUI(false);
								})
								.catch((e) => {
									setAwaitingUI(false);
									alert(e);	
								})
							})
							.catch((err) => {
								setAwaitingUI(false);
								alert(err);
							});
						}
					});
					elemUpdate.addEventListener("click", () => {
						const title = prompt("Введите новый заголовок поста:", p.title);
						if (!title) return;
						const content = prompt("Введите новое содержание поста:", p.content);
						if (!content) return;
						setAwaitingUI(true);
						updatePost(BigInt(p.id), title, content)
						.then((res) => {
							renderPosts()
							.then((posts) => {
								setAwaitingUI(false);
							})
							.catch((e) => {
								setAwaitingUI(false);
								alert(e);	
							})
						})
						.catch((err) => {
							setAwaitingUI(false);
							alert(err);
						});
						
					});	
				}
				else {
					elemUDelete.classList.add(UNVISIBLE_CLASS_NAME);
					elemUpdate.classList.add(UNVISIBLE_CLASS_NAME);
				}
			});
		}
		setAwaitingUI(false);
	})
	.catch((e) => {
		setAwaitingUI(false);
		alert(e);
	});

}

function renderPost(id, id2, title, author, content) {
	return `
	<div class="post__container">
		<p class="post__props">Тема: ${title}</p>
		<p class="post__props">Автор (код автора): ${author}</p>
		<textarea class="post__content" rows="3" disabled>${content}</textarea>
		<div class="post__menu-container">
			<span id="${id2}" class="post__button">Редактировать</span>
			<span id=${id} class="post__button">Удалить</span>
		</div>
	</div>
	`
}

function navigate(formName) {
	switch (formName) {
		case LOGIN_FORM_NAME: {
			posts_form.classList.add(UNVISIBLE_CLASS_NAME);
			login_form.classList.remove(UNVISIBLE_CLASS_NAME);
			reg_form.classList.add(UNVISIBLE_CLASS_NAME);
			break;
		}
		case REGISTER_FORM_NAME: {
			posts_form.classList.add(UNVISIBLE_CLASS_NAME);
			login_form.classList.add(UNVISIBLE_CLASS_NAME);
			reg_form.classList.remove(UNVISIBLE_CLASS_NAME);
			break;
		}
		case POSTS_FORM_NAME: {
			posts_form.classList.remove(UNVISIBLE_CLASS_NAME);
			login_form.classList.add(UNVISIBLE_CLASS_NAME);
			reg_form.classList.add(UNVISIBLE_CLASS_NAME);
			break;
		}
		default: {
			posts_form.classList.remove(UNVISIBLE_CLASS_NAME);
			login_form.classList.add(UNVISIBLE_CLASS_NAME);
			reg_form.classList.add(UNVISIBLE_CLASS_NAME);
		}
	}

}

function load() {
	setLinksBetweenAuthForms();
	setActionListeners();
	navigate(POSTS_FORM_NAME);
	renderPosts();
}

function setAwaitingUI(is_waiting) {
	is_waiting ? preloader.classList.remove(UNVISIBLE_CLASS_NAME) : preloader.classList.add(UNVISIBLE_CLASS_NAME);
}

async function tryToGetCurrentUser() {
	getUserInfo()
	.then((res) => {
		currentUser = res;
	})
	.catch((err) => {
		alert(err);
		currentUser = {};
	})
}

document.addEventListener('DOMContentLoaded', () => {
	setAwaitingUI(true);
	setTimeout(load, 3000);//ждем загрузку wasm модуля 3 сек.
	setAwaitingUI(true);
});

// API
let UserRegister = async () => {
	const username = document.getElementById("username_input_reg").value.trim();
	const email = document.getElementById("email_input_reg").value.trim();
	const password = document.getElementById("password_input_reg").value;
	if (!username || !email || !password) {
		alert("Bведите имя пользователя, email, пароль.")
		return;
	};
	return await window.wasmBindings.register(username, email, password);
};

let UserLogin = async () => {
	const email = document.getElementById("email_input_login").value.trim();
	const password = document.getElementById("password_input_login").value;
	if (!email || !password) {
		alert("Bведите email, пароль.")
		return;
	};
	return await window.wasmBindings.login(email, password);
}

let newPost = async (title, content) => {

	if (!title || !title.trim()) {title = "Без темы..."};
	if (!content || !content.trim()) {content = "Без содержания..."};
	return await window.wasmBindings.new_post(title, content);

}

let updatePost = async (id, title, content) => {
	if (!title || !title.trim()) {title = "Без темы..."};
	if (!content || !content.trim()) {content = "Без содержания..."};
	return await window.wasmBindings.update_post(id, title, content);
}

let delPost = async (postId) => {
	return await window.wasmBindings.del_post(postId);
}

let loadPosts = async () => {
	return await window.wasmBindings.posts(BigInt(20), BigInt(0));
}

let getUserInfo = async () => {
	return await window.wasmBindings.get_user_info();
}