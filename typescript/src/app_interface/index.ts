import { BrowserType, checkBrowserType } from "../util/mob";

export function hello(){
  return "Hello"
}

function osCheck(){

}

interface MapCallback {
  android: ()=>void
  ios: ()=>void
  browser: ()=>void
}

interface MapDetailCallback { 
  android_webview: ()=>void;
  android_mobile: ()=>void;
  ios_webview: ()=>void;
  ios_mobile: ()=>void;
  browser: ()=>void;
}

function runCallback(map: MapCallback){
  const browserType = checkBrowserType();
  if(browserType === BrowserType.Android_Webview) {
    
  } else if (browserType === BrowserType.iOS_Webview) {

  } else {

  }
}

function runDetailCallback(map: MapDetailCallback){
  const browserType = checkBrowserType()
}

function getKV(){
  runCallback({
    android: ()=>{},
    ios: ()=> {},
    browser: ()=> {},
  })
}

function setKV(){

}