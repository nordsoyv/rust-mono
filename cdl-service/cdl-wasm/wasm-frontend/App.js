import React from "react";
import produce from 'immer'
import * as wasm from "cdl-wasm";

// wasm.greet("sdf");
// let a = wasm.lex("config hub");
// alert(JSON.stringify(a));

const defaultState = {
    parseTime: 0,
    cdl: "",
    result: "",
}

const defaultCtx = {
    state: defaultState,
    dispatch: () => {
    }
}

const State = React.createContext(defaultCtx);

const reducer = produce((draft, {type, payload}) => {
    switch (type) {
        case "SET_CDL":
            draft.cdl = payload;
            break;
        case "SET_RESULT":
            draft.result = payload.text;
            draft.parseTime = payload.timeTaken;
            break;
    }
    return draft
})

const StateProvider = ({children}) => {
    const [state, dispatch] = React.useReducer(reducer, defaultState);

    return <State.Provider value={{state, dispatch}}>{children}</State.Provider>
}

const InputArea = () => {
    let ctx = React.useContext(State);
    // console.log(ctx);
    const onChangeHandler = (e) => {
        const v = e.target.value;
        ctx.dispatch({type: "SET_CDL", payload: v});
    }
    return <textarea cols="80" rows="40" onChange={onChangeHandler} value={ctx.state.cdl}></textarea>
}

const OutputArea = () => {
    let ctx = React.useContext(State);
    return <textarea cols="80" rows="40" value={ctx.state.result}/>
}

const LexButton = () => {
    let ctx = React.useContext(State);
    const onClickHandler = () => {
        const start = performance.now()
        let r = wasm.lex(ctx.state.cdl);
        const end = performance.now()
        ctx.dispatch({
            type: "SET_RESULT",
            payload: {
                text: JSON.stringify(r, null, '  '),
                timeTaken: end - start
            }
        })
    }
    return <button onClick={onClickHandler}>Lex</button>
}

const ParseButton = () => {
    let ctx = React.useContext(State);
    const onClickHandler = () => {
        const start = performance.now()
        let r = wasm.parse(ctx.state.cdl);
        const end = performance.now()
        ctx.dispatch({
            type: "SET_RESULT",
            payload: {
                text: JSON.stringify(r, null, '  '),
                timeTaken: end - start
            }
        })
    }
    return <button onClick={onClickHandler}>Parse</button>
}

const StatsDisplay = ()=> {
    let ctx = React.useContext(State);

    return <div>
        parseTime : {ctx.state.parseTime.toFixed(2)} ms
    </div>

}

export function App() {
    return (
        <StateProvider>
            <div className="App">
                <StatsDisplay/>
                <InputArea/>
                <LexButton/>
                <ParseButton/>
                <button id="lexWsButton">Lex WS</button>
                <button id="parseWsButton">Parse WS</button>

                <OutputArea/>
            </div>
        </StateProvider>
    );
}
