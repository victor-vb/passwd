let hostname = window.location.hostname;

document.addEventListener("keydown", function (event) {
    let changeev = new Event('change', { bubbles: true });
    let mousedownev = new Event('mousedown', { bubbles: true });
    if (event.metaKey && event.shiftKey && event.key=='s'){
        console.log("select all")
        document.querySelectorAll("input[type='checkbox']").forEach((input) => { if (!input.checked) { input.click() } });
        let minLimit = document.getElementById("minLimit")
        if(minLimit){
            minLimit.value = 50;
            minLimit.dispatchEvent(changeev);
        }
        
        let maxLimit = document.getElementById("maxLimit");
        if(maxLimit){
            maxLimit.value = 5000;
            maxLimit.dispatchEvent(changeev);
        }
        let quotaLimit = document.getElementById("quotaLimit");
        if(quotaLimit){
            quotaLimit.value = 1000000;  
            quotaLimit.dispatchEvent(changeev);
        }
        // document.getElementById("financeApiId").setAttribute("aria-activedescendant","financeApiId_list_1");
        document.getElementById("financeApiId").click();
        document.getElementById("status").getElementsByTagName("input")[0].click(); 
    }
    if (event.metaKey && event.shiftKey && event.key == 'e') {
        console.log("cancel all")
        document.querySelectorAll("input[type='checkbox']").forEach((input) => { if (input.checked) { input.click() } });
    }

    if (event.metaKey && event.shiftKey && event.key == 'u') {
        console.log("get config from file")
        chrome.runtime.sendMessage({ "hostname": hostname, "account": '',config:1 }).then((resposne) => {
            try {
                let rc_select_15_list = document.getElementById("rc_select_15");
                if (rc_select_15_list){
                    let pages = document.getElementById("rc_select_15").parentElement.parentElement.parentElement.parentElement.parentElement.getElementsByClassName("ant-pagination-item");
                    pages[pages.length-1].click();
                    let payId = document.getElementById("payId");
                    if (payId){
                        let id = document.getElementsByClassName("myXTableView")[0].getElementsByClassName("ant-table-body")[0].querySelectorAll("tr:last-child")[0].querySelector("td:nth-child(2)").textContent;
                        payId.value = parseInt(id)+1;
                        payId.dispatchEvent(changeev);
                    }
                }
                let config = resposne.passwds[0];
                let code = document.querySelector("input[name='code']");
                if (code){
                    code.value = config.code;
                }
                let title = document.querySelector("input[name='title']");
                if (title) {
                    title.value = config.title;
                }
                let cipher = document.querySelector("input[name='cipher']");
                if (cipher) {
                    cipher.value = config.cipher;
                }
                let flags = document.querySelector("select[name='flags']");
                if (flags) {
                    flags.children[2].selected = true;
                }
                let content = document.querySelector("textarea[name='content']");
                if (content) {
                    content.value = config.content;
                }

                let payName = document.getElementById("payName");
                if (payName) {
                    payName.value = config.code;
                    payName.dispatchEvent(changeev);
                }
                let payInnerName = document.getElementById("payInnerName");
                if (payInnerName) {
                    payInnerName.value = config.title;
                    payInnerName.dispatchEvent(changeev);
                }

                document.querySelectorAll("input[type='checkbox']").forEach((input) => { if (!input.checked) { input.click() } });
                document.getElementById("onLine");
            
                let onLine = document.getElementById("onLine");
                if (onLine){
                    onLine.dispatchEvent(mousedownev);
                    setTimeout(() => {
                        document.getElementById("onLine_list").parentElement.querySelectorAll(".ant-select-item-option")[0].click();
                    }, 50);
                }
                let isNew = document.getElementById("isNew");
                if (isNew) {
                    isNew.dispatchEvent(mousedownev);  
                    setTimeout(() => {
                        document.getElementById("isNew_list").parentElement.querySelectorAll(".ant-select-item-option")[1].click();
                    }, 50);
                }

                console.table(config); 
               
            } catch (error) {
                console.warn(error);
            }
        }).catch((reason) => {
            console.error(reason);
        });
    }

    let inputs = get_input_list();
    if (inputs.length < 2) {
        return false;
    }
    let account_value = inputs[0].value;
    if (event.metaKey && (event.key == 'i')) {
        chrome.runtime.sendMessage({ "hostname": hostname, "account": '' }).then((resposne) => {
            try {
                let accounts = resposne.passwds;
                console.table(accounts);
                let account = null;
                if (accounts.length >= 1) {
                    for (let index = 0; index < accounts.length; index++) {
                        if (account_value) {
                            if (account_value == accounts[index].username) {
                                account = accounts[index];
                            }
                        } else {
                            account = accounts[index];
                            break;
                        }
                    }
                    account = account ? account : accounts[0];
                    input_username_passwd(account.username, account.password, account.code);
                    attach_listen(accounts);
                }
            } catch (error) {
                console.warn(error);
            }
        }).catch((reason) => {
            console.error(reason);
        });
    }

    let passwds_elem = document.getElementById("password_list");
    if (!passwds_elem) {
        return false;
    }
    let passwds = JSON.parse(passwds_elem.getAttribute("data-passwds"));

    if (!account_value) {
        return false;
    }

    if (event.key == 'ArrowDown' || event.key == 'ArrowUp') {
        passwds_elem.style.display = "block";
        var mouseenter = new Event('mouseenter', { bubbles: true });
        var mouseleave = new Event('mouseleave', { bubbles: true });
        let spans = passwds_elem.getElementsByTagName("span");
        let isset = false;
        for (let index = 0; index < spans.length; index++) {
            let element = spans[index];
            if (!element.classList.contains("antiquewhite") && !isset) {
                account_value = element.textContent;
                element.dispatchEvent(mouseenter);
                isset = true;
            } else {
                element.dispatchEvent(mouseleave);
            }
        }
    }

    if (event.key == 'ArrowLeft' || event.key == 'ArrowRight') {
        passwds_elem.style.display = "none";
    }

    let account = null;
    for (let index = 0; index < passwds.length; index++) {
        if (passwds[index].username == account_value) {
            account = passwds[index];
            break;
        }
    }

    if (!account) {
        return false;
    }

    let { username, password, code } = account;
    if (event.metaKey && event.key == 'c' && username) {
        copy_text(username);
        show_message(`已复制账户`);
        event.preventDefault();
    }

    if (event.metaKey && event.key == 'x' && password) {
        copy_text(password);

        show_message(`已复制密码`);
        event.preventDefault();
    }

    if (event.metaKey && event.key == 'z' && code) {
        copy_text(code);
        show_message(`已复制2FA验证码:${code}`);
        event.preventDefault();
    }

    input_username_passwd(username, password);
});


function input_username_passwd(username, password, google2fa) {
    let inputs = get_input_list();
    let event = new Event('change', { bubbles: true });
    let input = new Event('input', { bubbles: true });

    if (inputs.length >= 2) {
        let user_account = inputs[0];
        let password_input = inputs[1];
        user_account.value = username;
        user_account.dispatchEvent(event);
        user_account.dispatchEvent(input);
        if (password_input.type == 'password') {
            password_input.value = password;
            password_input.dispatchEvent(event);
            password_input.dispatchEvent(input);
        }
        if (inputs.length >= 3 && google2fa) {
            let google_input = inputs[2];
            google_input.value = google2fa;
            google_input.dispatchEvent(event);
            google_input.dispatchEvent(input);
        }
    }
}

function show_message(message) {
    let toast = document.getElementById("toast_message");
    if (!toast) {
        toast = document.createElement("div");
    }
    toast.id = "toast_message";
    toast.classList.add("animate__fadeIn");
    toast.textContent = message;
    toast.style.display = "block"
    toast.addEventListener('animationend', handleAnimationEnd, { once: true });
    document.body.append(toast);
}

function handleAnimationEnd(event) {
    event.stopPropagation();
    event.target.classList.remove("animate__fadeIn");
    event.target.classList.add("animate__fadeOutRight");
    setTimeout(() => {
        event.target.classList.remove("animate__fadeOutRight");
        event.target.style.display = "none";
    }, 500);
}

function copy_text(text) {
    // 数字没有 .length 不能执行selectText 需要转化成字符串
    let input = document.querySelector('#copy-input');
    if (!input) {
        input = document.createElement('input');
        input.id = "copy-input";
        input.readOnly = "readOnly";        // 防止ios聚焦触发键盘事件
        input.style.position = "absolute";
        input.style.left = "-1000px";
        input.style.zIndex = "-1000";
        document.body.appendChild(input)
    }

    input.value = text;
    // ios必须先选中文字且不支持 input.select();
    selectText(input, 0, text.length);
    if (document.execCommand('copy')) {
        // document.execCommand('copy');
    }
    input.blur();

    // input自带的select()方法在苹果端无法进行选择，所以需要自己去写一个类似的方法
    // 选择文本。createTextRange(setSelectionRange)是input方法
    function selectText(textbox, startIndex, stopIndex) {
        if (textbox.createTextRange) {//ie
            const range = textbox.createTextRange();
            range.collapse(true);
            range.moveStart('character', startIndex);//起始光标
            range.moveEnd('character', stopIndex - startIndex);//结束光标
            range.select();//不兼容苹果
        } else {//firefox/chrome
            textbox.setSelectionRange(startIndex, stopIndex);
            textbox.focus();
        }
    }
}

function attach_listen(accounts) {
    let account_elem = get_input_list()[0];
    let password_list = document.getElementById("password_list");
    if (password_list) {
        password_list.remove();
    }
    let top = account_elem.offsetHeight + account_elem.offsetTop + 2;
    let contanier = document.createElement("div");
    contanier.id = "password_list"
    contanier.dataset.passwds = JSON.stringify(accounts);
    accounts.forEach(account => {
        let span = document.createElement("span");
        span.textContent = account.username;
        span.addEventListener("click", function (event) {
            contanier.style.display = "none";
            input_username_passwd(event.target.textContent, account.password, account.code);
        });

        span.addEventListener("mouseenter", function (event) {
            event.target.classList.add("antiquewhite");
        });

        span.addEventListener("mouseleave", function (event) {
            event.stopPropagation();
            event.target.classList.remove("antiquewhite");
        });

        contanier.appendChild(span);
    });

    contanier.addEventListener("touchstart", (event) => {
        var mouseenter = new Event('mouseenter', { bubbles: true });
        var mouseleave = new Event('mouseleave', { bubbles: true });
        let spans = event.target.parentNode.getElementsByTagName("span");
        for (let index = 0; index < spans.length; index++) {
            let elem = spans[index];
            if (elem !== event.target) {
                elem.dispatchEvent(mouseleave);
            } else {
                elem.dispatchEvent(mouseenter);
            }
        }
    });

    account_elem.parentNode.insertBefore(contanier, account_elem.nextSibling);
    contanier.style.top = top + "px";


    contanier.addEventListener("mouseleave", (event) => {
        contanier.style.display = "none";
    });
    contanier.addEventListener("mouseenter", (event) => {

        let spans = event.target.getElementsByTagName("span");
        for (let span of spans) {
            span.classList.remove("antiquewhite");
        }
    });

    account_elem.addEventListener("focus", function (dom, event) {
        contanier.style.display = "block";
    });
}



function get_input_list() {
    let parentNode = sure_parent_node();;
    let inputs = []
    if (!parentNode) {
        return inputs;
    }
    let inputs_object = parentNode.getElementsByTagName("input");

    for (let i = 0; i < inputs_object.length; i++) {
        let input = inputs_object[i];
        if (["text", "password", "tel"].indexOf(input.type) !== -1) {
            inputs.push(input);
        }
    }
    return inputs;
}

function sure_parent_node(parentNode = null, current_iter = 0) {
    try {
        if (current_iter >= 100) {
            return parentNode;
        }
        if (!parentNode) {
            let inputs_object = document.querySelector("input[type='password']");
            parentNode = inputs_object.parentNode;
        } else {
            parentNode = parentNode.parentNode;
            if (parentNode.tagName == 'FORM'){
                return parentNode;
            }
        }
        let length = 0;
        let inputs = parentNode.getElementsByTagName("input");
        for (let index = 0; index < inputs.length; index++) {
            let input = inputs[index];
            if (input.style.display == "none" || input.type == 'hidden' || input.type =="submit"){
                continue;
            }
            length++;
        }
        if (length <= 2) {
            return sure_parent_node(parentNode);
        }
    } catch (error) {
        // console.log(`无法找到上级列表`, error);
    }

    return parentNode;
}