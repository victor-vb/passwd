import decrypt from "./assets/js/aes.js"

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    load_data(message.hostname, message.account,message.config).then((response) => {
        try {
            let data = decrypt(response);
            data = JSON.parse(data);
            sendResponse(data);
        } catch (error) {
            console.warn("解析发生错误！");
        }
       
    });
    return true;
});



async function load_data (url,account,config){
    let headers = new Headers();
    let hostname = "127.0.0.1:8083";
    if (config){
        hostname = "127.0.0.1:8082";
    }
    headers.append('Content-Type', 'application/text');
    headers.append('Accept', 'application/text');
    headers.append('Access-Control-Allow-Origin', hostname);
    headers.append('GET', 'POST', 'OPTIONS');
    let host = `http://${hostname}/passwd`;
    let data = await fetch(`${host}?host=${url}&account=${account}&config=${config}`, {
        method: 'GET',
        headers: headers
    });
    let text = await data.text();
    return text;
}

chrome.sidePanel
    .setPanelBehavior({ openPanelOnActionClick: true })
    .catch((error) => console.error(error));