import * as PIXI from 'pixi.js';
import sampleImage from '../../static/image/test_image.png';
import tree from '../../static/image/Tree1.png';
import test from '../../static/image/test.png';

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
    bunny.zIndex = 1;
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

//
//
//
createTree(50, 50);
createTree(150, 150);
let treeDragTarget: PIXI.Sprite | null;

function createTree(x: number, y: number){
  const texture = PIXI.Texture.from(tree);
  texture.baseTexture.scaleMode = PIXI.SCALE_MODES.NEAREST;
  const sprite = new PIXI.Sprite(texture);
  sprite.anchor.set(0.5);
  sprite.scale.set(3);
  sprite.on('pointerdown', onTreeDragStart);
  sprite.x = x;
  sprite.y = y;
  app.stage.addChild(sprite);
}

function onTreeDragStart(this: PIXI.Sprite, event: any){
  this.alpha = 0.5;
  treeDragTarget = this;
  app.stage.on('pointermove', onTreeDragMove);
}

function onTreeDragMove(event: any){
  if(treeDragTarget) {
    treeDragTarget.parent?.toLocal(event.data.global, undefined, treeDragTarget.position)
  }
}
//
//
createTest(300, 300);
function createTest(x: number, y: number){
  const texture = PIXI.Texture.from(test);
  texture.baseTexture.scaleMode = PIXI.SCALE_MODES.NEAREST;
  const sprite = new PIXI.Sprite(texture);
  sprite.anchor.set(0.5);
  sprite.scale.set(3);
  sprite.x = x;
  sprite.y = y;
  sprite.zIndex = 10;
  app.stage.addChild(sprite);
}

// let sprite = PIXI.Sprite.from(sampleImage, {height:100, width:100});
// sprite.on('click', ()=>{
//   console.log('good')
// })
// sprite.height = 100
// sprite.width = 100
// sprite.eventMode = 'static'

// app.stage.addChild(sprite); 

// // 버튼 생성
// const button = new PIXI.Graphics();
// button.beginFill(0x3498db); // 버튼 색상
// button.drawRect(0, 0, 100, 50); // 버튼 크기
// button.endFill();
// button.interactive = true; // 이벤트 활성화

// // 버튼에 텍스트 추가
// const buttonText = new PIXI.Text('Click me', {
//   fill: 0xFFFFFF, // 텍스트 색상
//   fontSize: 16, // 텍스트 크기
//   fontWeight: 'bold', // 텍스트 굵기
// });
// buttonText.anchor.set(0.5, 0.5); // 텍스트의 중심을 기준으로 설정
// buttonText.position.set(button.width / 2, button.height / 2); // 텍스트 위치 설정
// button.addChild(buttonText);

// // 클릭 이벤트 처리
// button.on('pointerdown', () => {
//   console.log('버튼이 클릭되었습니다!');
//   // buttonText.alpha = 0
// });

// app.stage.addChild(button);