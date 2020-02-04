import React from 'react';
import ReactDOM from 'react-dom';
import {App} from './App'
import axios from 'axios';


// wasm.greet("sdf");
// let a = wasm.lex("config hub");
// alert(JSON.stringify(a));


ReactDOM.render(
    <App/>,
document.getElementById('app')
);
/*
let timeTaken = document.getElementById("timeTaken");
let textArea = document.getElementById("input");
let lexButton = document.getElementById("lexButton");
let parseButton = document.getElementById("parseButton");
let lexWsButton = document.getElementById("lexWsButton");
let parseWsButton = document.getElementById("parseWsButton");
let output = document.getElementById("output");

function postData(url, data) {
    return axios.post(url, data, {
        headers: {
            'Access-Control-Allow-Origin': '*',
            "Content-Type": 'application/json',
        }
    })
    //return fetch(url, {
    //     method: "POST",
    //     mode: 'no-cors',
    //     credentials: 'same-origin',
    //     body: JSON.stringify(data),
    //     headers: {
    //         'Accept': 'application/json',
    //         'Content-Type': 'application/json',
    //     },
    // })
}

lexButton.addEventListener("click", e => {
    const startTime = new Date();
    let lexed = wasm.lex(textArea.value);
    const endTime = new Date();
    output.value = JSON.stringify(lexed, null, "  ")
    const endJSON = new Date();

    timeTaken.innerText = "Time taken to lex: "
        + (endTime - startTime).toString() + "\nTime taken to JSON: "
        + (endJSON - endTime).toString() + "\nTotal time: " + (endJSON - startTime).toString();
})

parseButton.addEventListener("click", e => {
    const startTime = new Date();
    let ast = wasm.parse(textArea.value);
    //output.innerText = JSON.stringify(ast)
    const endTime = new Date();
    output.value = JSON.stringify(ast, null, "  ")

    const endJSON = new Date();
    timeTaken.innerText = "Time taken to parse: "
        + (endTime - startTime).toString() + "\nTime taken to JSON: "
        + (endJSON - endTime).toString() + "\nTotal time: " + (endJSON - startTime).toString();
})


lexWsButton.addEventListener('click', () => {
    const startTime = new Date();
    // console.log(JSON.stringify({cdl: textArea.value}));
    postData("http://localhost:8081/lex", {cdl: textArea.value}).then(res => {
        const endTime = new Date();
        output.value = JSON.stringify(res.data, null, "  ")
        const endJSON = new Date();
        timeTaken.innerText = "Time taken to lex WS: "
            + (endTime - startTime).toString() + "\nTime taken to JSON: "
            + (endJSON - endTime).toString() + "\nTotal time: " + (endJSON - startTime).toString();


    }).catch(e => console.log(e));
});

parseWsButton.addEventListener('click', () => {
    const startTime = new Date();
    // console.log(JSON.stringify({cdl: textArea.value}));
    postData("http://localhost:8081/parse", {cdl: textArea.value}).then(res => {
        const endTime = new Date();
        console.log("asdfasdf");
        output.value = JSON.stringify(res.data, null, "  ")
        const endJSON = new Date();
        timeTaken.innerText = "Time taken to parse WS: "
            + (endTime - startTime).toString() + "\nTime taken to JSON: "
            + (endJSON - endTime).toString() + "\nTotal time: " + (endJSON - startTime).toString();


    }).catch(e => console.log(e));
});

*/
