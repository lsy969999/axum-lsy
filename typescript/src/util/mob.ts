/**
 * BrowserType: 브라우저 타입
 * iOS_Webview: 아이폰이면서 내 앱의 웹뷰 브라우저  
 * iOS_Mobile: 아이폰이면서 타앱의 웹뷰 브라우저  
 * Android_Webview: 안드로이드이면서 내앱의 웹뷰 브라우저  
 * Android_Mobile: 안드로이드이면서 타앱의 웹뷰 브라우저  
 * Brower: 피시브라우저  
 * Others: 이외의 브라우저  
*/
export enum BrowserType {
  iOS_Webview,
  iOS_Mobile,
  Android_Webview,
  Android_Mobile,
  Brower,
  Others
}

/**
 * 내 앱인지 판단할 userAgen문구
 */
export const myAppUseraAgent = "myApp";

export function checkBrowserType(): BrowserType{
  const isAndroid = checkAndroid();
  const isiOS = checkiOS();
  const isBrowser = checkBrowser();
  
  if(isAndroid) {
    if(checkMyApp()){
      return BrowserType.Android_Webview;
    } else {
      return BrowserType.Android_Mobile;
    }
  } else if (isiOS) {
    if(checkMyApp()){
      return BrowserType.iOS_Webview;
    } else {
      return BrowserType.iOS_Mobile;
    }
  } else if (isBrowser) {
    return BrowserType.Brower;
  } else {
    return BrowserType.Others;
  }
}

function checkAndroid() {
  return /Android/.test(navigator.userAgent);
}

function checkiOS(){
  return /(iPad|iPhone|iPod)/.test(navigator.userAgent);
}

function checkBrowser(){
  return !(/(Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini)/.test(navigator.userAgent));
}

function checkMyApp(){
  const regex = new RegExp(`(${myAppUseraAgent})`);
  return regex.test(navigator.userAgent);
}