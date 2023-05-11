import React from 'react';
import './App.css';
import {Box, Container, Typography} from "@mui/material";
import {GameState} from "../game-core/pkg/game_core";

export function App({gameState}: { gameState: GameState | null }) {
  return (
    <Container maxWidth="sm">
      <Box sx={{my: 4}}>
        <Typography variant="h4" component="h1" gutterBottom>
          Factorio Idle
        </Typography>
        {gameState && gameState.counter}
      </Box>

    </Container>
  );
}
