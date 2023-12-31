import * as PIXI from 'pixi.js';

const app = new PIXI.Application<HTMLCanvasElement>({
    background: '#1099bb',
    resizeTo: window,
});

const domApp = document.getElementById('pixiApp')!;
domApp.appendChild(app.view);

const basicText = new PIXI.Text('Basic text in pixi1');
basicText.x = 50;
// basicText.y = 100;
app.stage.addChild(basicText);

const basicText2 = new PIXI.Text('Basic text in pixi2');
basicText2.x = 50;
basicText2.y = 100;
app.stage.addChild(basicText2);

const texture = PIXI.Texture.from('https://pixijs.com/assets/bunny.png');
texture.baseTexture.scaleMode = PIXI.SCALE_MODES.NEAREST;

for (let i = 0; i < 10; i++) {
    createBunny(
        Math.floor(Math.random() * app.screen.width),
        Math.floor(Math.random() * app.screen.height),
    );
}

function createBunny(x: number, y: number) {
    const bunny = new PIXI.Sprite(texture);
    bunny.interactive = true;
    // bunny.buttonMode = true;
    bunny.anchor.set(0.5);
    bunny.scale.set(3);
    bunny.on('pointerdown', onDragStart);
    bunny.x = x;
    bunny.y = y;
    app.stage.addChild(bunny);
}

let dragTarget: PIXI.Sprite | null = null;

app.stage.interactive = true;
app.stage.hitArea = new PIXI.Rectangle(0, 0, app.screen.width, app.screen.height);
app.stage.on('pointerup', onDragEnd);
app.stage.on('pointerupoutside', onDragEnd);

function onDragMove(event: any) {
    if (dragTarget) {
        dragTarget.parent?.toLocal(event.data.global, undefined, dragTarget.position);
    }
}

function onDragStart(this: PIXI.Sprite, event: any) {
  this.alpha = 0.5;
  dragTarget = this;
  app.stage.on('pointermove', onDragMove);
}

function onDragEnd() {
    if (dragTarget) {
        app.stage.off('pointermove', onDragMove);
        dragTarget.alpha = 1;
        dragTarget = null;
    }
}

