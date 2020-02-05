import React from 'react';
import produce from 'immer';
import * as wasm from 'cdl-wasm';
import {getCompiler} from 'confirmit-cdl-compiler';
import './main.css';

const {Compiler} = getCompiler('v2');

// wasm.greet("sdf");
// let a = wasm.lex("config hub");
// alert(JSON.stringify(a));

const defaultState = {
  parseTime: 0,
  cdl: '',
  result: '',
};

const defaultCtx = {
  state: defaultState,
  dispatch: () => {},
};

const State = React.createContext(defaultCtx);

const reducer = produce((draft, {type, payload}) => {
  switch (type) {
    case 'SET_CDL':
      draft.cdl = payload;
      break;
    case 'SET_RESULT':
      draft.result = payload.text;
      draft.parseTime = payload.timeTaken;
      break;
  }
  return draft;
});

const StateProvider = ({children}) => {
  const [state, dispatch] = React.useReducer(reducer, defaultState);

  return <State.Provider value={{state, dispatch}}>{children}</State.Provider>;
};

const InputArea = () => {
  let ctx = React.useContext(State);
  // console.log(ctx);
  const onChangeHandler = e => {
    const v = e.target.value;
    ctx.dispatch({type: 'SET_CDL', payload: v});
  };
  return <textarea cols="80" rows="40" onChange={onChangeHandler} value={ctx.state.cdl}></textarea>;
};

const OutputArea = () => {
  let ctx = React.useContext(State);
  return <textarea cols="80" rows="40" value={ctx.state.result} />;
};

const LexButton = () => {
  let ctx = React.useContext(State);
  const onClickHandler = () => {
    const start = performance.now();
    let r = wasm.lex(ctx.state.cdl);
    const end = performance.now();
    ctx.dispatch({
      type: 'SET_RESULT',
      payload: {
        text: JSON.stringify(r, null, '  '),
        timeTaken: end - start,
      },
    });
  };
  return <button onClick={onClickHandler}>Lex WASM</button>;
};

const ParseButton = () => {
  let ctx = React.useContext(State);
  const onClickHandler = () => {
    const start = performance.now();
    let r = wasm.parse(ctx.state.cdl);
    const end = performance.now();
    ctx.dispatch({
      type: 'SET_RESULT',
      payload: {
        text: JSON.stringify(r, null, '  '),
        timeTaken: end - start,
      },
    });
  };
  return <button onClick={onClickHandler}>Parse WASM</button>;
};

const ParsePEGButton = () => {
  let ctx = React.useContext(State);
  const onClickHandler = () => {
    const compiler = new Compiler();
    const startParsing = performance.now();
    const ast = compiler.compile(ctx.state.cdl, {pipeLine: []});
    const endParsing = performance.now();
    const parseTime = endParsing - startParsing;
    ctx.dispatch({
      type: 'SET_RESULT',
      payload: {
        text: JSON.stringify(ast, null, '  '),
        timeTaken: parseTime,
      },
    });
  };
  return <button onClick={onClickHandler}>Parse PEG</button>;
};

const ParsePEGWithPluginsButton = () => {
  let ctx = React.useContext(State);
  const onClickHandler = () => {
    const compiler = new Compiler();
    const startParsing = performance.now();
    const ast = compiler.compile(ctx.state.cdl, {pipeLine: 'full'});
    const endParsing = performance.now();
    const parseTime = endParsing - startParsing;

    ctx.dispatch({
      type: 'SET_RESULT',
      payload: {
        text: JSON.stringify(ast, null, '  '),
        timeTaken: parseTime,
      },
    });
  };
  return <button onClick={onClickHandler}>Parse PEG with plugins</button>;
};

const StatsDisplay = () => {
  let ctx = React.useContext(State);

  return <div>parseTime : {ctx.state.parseTime.toFixed(2)} ms</div>;
};

const LexWs = () => {
  let ctx = React.useContext(State);
  const onClickHandler = () => {
    const startFetch = performance.now();
    fetch('http://localhost:8081/lex', {
      method: 'POST',
      body: JSON.stringify({
        cdl: ctx.state.cdl,
      }),
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Content-Type': 'application/json',
      },
    })
      .then(res => res.json())
      .then(res => {
        const endFetch = performance.now();
        const fetchTime = endFetch - startFetch;
        ctx.dispatch({
          type: 'SET_RESULT',
          payload: {
            text: JSON.stringify(res, null, '  '),
            timeTaken: fetchTime,
          },
        });
      })
      .catch(err => {
        console.log(err);
      });
  };

  return <button onClick={onClickHandler}>Lex WS</button>;
};

const ParseWs = () => {
  let ctx = React.useContext(State);
  const onClickHandler = () => {
    const startFetch = performance.now();
    let endFetch = 0;
    fetch('http://localhost:8081/parse', {
      method: 'POST',
      body: JSON.stringify({
        cdl: ctx.state.cdl,
      }),
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Content-Type': 'application/json',
      },
    })
      .then(res => {
        endFetch = performance.now();
        if (res.status === 200) {
          return res;
        }
        throw res;
      })
      .then(res => res.json())
      .then(res => {

        const fetchTime = endFetch - startFetch;
        ctx.dispatch({
          type: 'SET_RESULT',
          payload: {
            text: JSON.stringify(res, null, '  '),
            timeTaken: fetchTime,
          },
        });
      })
      .catch(err => err.text())
      .then(t => {
        endFetch = performance.now();
        const fetchTime = endFetch - startFetch;

        ctx.dispatch({
          type: 'SET_RESULT',
          payload: {
            text: t,
            timeTaken: fetchTime,
          },
        });
      });
  };

  return <button onClick={onClickHandler}>Parse WS</button>;
};
const Row = ({children}) => {
  return <div className="row">{children}</div>;
};

const Column = ({children}) => {
  return <div className="column">{children}</div>;
};

export function App() {
  return (
    <StateProvider>
      <Column>
        <StatsDisplay />
        <Row>
          <InputArea />
          <div style={{margin: 5}}>
            <Column>
              <LexButton />
              <ParseButton />
              <ParsePEGButton />
              <ParsePEGWithPluginsButton />
              <LexWs />
              <ParseWs />
            </Column>
          </div>
          <OutputArea />
        </Row>
      </Column>
    </StateProvider>
  );
}
