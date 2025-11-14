import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export namespace Minesweeper {
    export interface Cell {
        state: "Hidden" |
        { Revealed: number } |
        "Flagged" |
        "Bomb";
    }

    export interface DisplayBoard {
        difficulty: "easy" | "medium" | "hard";
        cells: Cell[][];
        game_state: "Ongoing" | "Won" | "Lost";
        time_elapsed: number | null;
    }
}

export class Game {
    private _onClick?: (e: MouseEvent) => void;
    private _onRightClick?: (e: MouseEvent) => void;

    gameState: Minesweeper.DisplayBoard | null = $state(null);
    board: HTMLCanvasElement | null = $state(null);
    cellSize: number = 40;
    fontSize: number = 20;

    unlisten: (() => void) | null = null;

    constructor() {
        listen<Minesweeper.DisplayBoard>("board:update", (event) => {
            const updatedBoard = event.payload;
            this.setGameState(updatedBoard);
        }).then((unlisten) => {
            this.unlisten = unlisten;
        });
    }

    public destroy() {
        if (this.unlisten) {
            this.unlisten();
            this.unlisten = null;
        }
    }

    public setGameState(gameState: Minesweeper.DisplayBoard) {
        this.gameState = gameState;
        this.cellSize = Math.min(40, Math.floor((this.board?.width ?? 600) / gameState.cells[0].length));
        this.fontSize = Math.floor(this.cellSize * 0.5);
        this.render();
    }
    public setBoard(board: HTMLCanvasElement) {
        if (this.board) {
            if (this._onClick) {
                this.board.removeEventListener("click", this._onClick);
            }
            if (this._onRightClick) {
                this.board.removeEventListener("contextmenu", this._onRightClick);
            }
        }
        this.board = board;
        this._onClick = this.handleCanvasClick.bind(this);
        this._onRightClick = this.handleRightClick.bind(this);
        this.board.addEventListener("click", this._onClick);
        this.board.addEventListener("contextmenu", this._onRightClick);
    }

    public getDuration(): number {
        if (!this.gameState || this.gameState.time_elapsed === null) {
            return 0;
        }
        return this.gameState.time_elapsed
    }

    public render() {
        if (!this.board || !this.gameState) {
            console.error("Board or game state is not set");
            return;
        }
        const ctx = this.board.getContext("2d");
        if (!ctx) return;

        // Set font and alignment for text
        ctx.font = `${this.fontSize}px Arial`;
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        //clear the canvas
        ctx.clearRect(0, 0, this.board.width, this.board.height);

        for (let y = 0; y < this.gameState.cells.length; y++) {
            for (let x = 0; x < this.gameState.cells[y].length; x++) {
                const cell = this.gameState.cells[y][x];
                const centerX = x * this.cellSize + this.cellSize / 2; // Center X
                const centerY = y * this.cellSize + this.cellSize / 2; // Center Y

                // Draw the cell background
                switch (cell.state) {
                    case "Hidden":
                        ctx.fillStyle = "#fff"; // White for hidden cells
                        break;
                    case "Flagged":
                        ctx.fillStyle = "#ff0000";
                        break;
                    case "Bomb":
                        ctx.fillStyle = "#ff0000";
                        break;
                    default:
                        if (typeof cell.state === "object" && "Revealed" in cell.state) {
                            ctx.fillStyle = "#ccc"; // Gray for revealed cells
                        } else {
                            console.error("Unknown cell state:", cell.state);
                            continue;
                        }
                }
                ctx.fillRect(x * this.cellSize, y * this.cellSize, this.cellSize, this.cellSize);
                ctx.strokeRect(x * this.cellSize, y * this.cellSize, this.cellSize, this.cellSize);

                // Draw the text (if applicable)
                if (cell.state === "Flagged") {
                    ctx.fillStyle = "#000"; // Black text for flagged cells
                    ctx.fillText("🏳", centerX, centerY);
                } else if (cell.state === "Bomb") {
                    ctx.fillStyle = "#000"; // Black text for bombs
                    ctx.fillText("💣", centerX, centerY);
                } else if (typeof cell.state === "object" && "Revealed" in cell.state) {
                    ctx.fillStyle = "#000"; // Black text for revealed numbers
                    ctx.fillText(cell.state.Revealed.toString(), centerX, centerY);
                }
            }
        }
    }

    handleCanvasClick(event: MouseEvent) {
        if (!this.board || !this.gameState) return;
        const rect = this.board.getBoundingClientRect();
        const mouseX = event.clientX - rect.left;
        const mouseY = event.clientY - rect.top;

        const col = Math.floor(mouseX / this.cellSize);
        const row = Math.floor(mouseY / this.cellSize);

        invoke<Minesweeper.DisplayBoard>('reveal_cell', { row, col }).then((updatedBoard: Minesweeper.DisplayBoard) => {
            this.setGameState(updatedBoard);
            this.render();
        });
    }

    handleRightClick(event: MouseEvent) {
        event.preventDefault();
        event.stopPropagation();
        if (!this.board || !this.gameState) return;
        const rect = this.board.getBoundingClientRect();
        const mouseX = event.clientX - rect.left;
        const mouseY = event.clientY - rect.top;

        const col = Math.floor(mouseX / this.cellSize);
        const row = Math.floor(mouseY / this.cellSize);

        console.log(`Right-clicked on cell: (${row}, ${col})`);
        invoke<Minesweeper.DisplayBoard>('flag_cell', { row, col }).then((updatedBoard: Minesweeper.DisplayBoard) => {
            this.setGameState(updatedBoard);
            this.render();
        });
    }
}