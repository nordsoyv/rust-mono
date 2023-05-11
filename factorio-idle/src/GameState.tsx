import React, { useEffect, useState } from 'react';
import { App } from './App.tsx';
import { GameState } from '../game-core/pkg/game_core';
import {CssBaseline, ThemeProvider} from "@mui/material";
import {theme} from './theme';

function useForceUpdate() {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [_value, setValue] = useState(0); // integer state
  return () => setValue((value) => ++value); // update the state to force render
}
export const GameStateContext = ({ gameState }: { gameState: GameState }) => {
  const forceUpdate = useForceUpdate();
  useEffect(() => {
    setInterval(() => {
      if (gameState != null) {
        gameState.tick(1);
        forceUpdate();
      }
    }, 100);
  }, []);

  return (
    <React.StrictMode>
      <ThemeProvider theme={theme}>
        <CssBaseline/>
        <App gameState={gameState} />
      </ThemeProvider>

    </React.StrictMode>
  );
};
