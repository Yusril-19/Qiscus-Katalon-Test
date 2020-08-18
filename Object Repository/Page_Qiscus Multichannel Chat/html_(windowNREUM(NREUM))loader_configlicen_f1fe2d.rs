<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_(windowNREUM(NREUM))loader_configlicen_f1fe2d</name>
   <tag></tag>
   <elementGuidId>ac1fae1d-0215-4b8a-a924-f3c731aa79e3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    (window.NREUM||(NREUM={})).loader_config={licenseKey:&quot;b9f2942ac8&quot;,applicationID:&quot;246424479&quot;};window.NREUM||(NREUM={}),__nr_require=function(e,n,t){function r(t){if(!n[t]){var i=n[t]={exports:{}};e[t][0].call(i.exports,function(n){var i=e[t][1][n];return r(i||n)},i,i.exports)}return n[t].exports}if(&quot;function&quot;==typeof __nr_require)return __nr_require;for(var i=0;i&lt;t.length;i++)r(t[i]);return r}({1:[function(e,n,t){function r(){}function i(e,n,t){return function(){return o(e,[u.now()].concat(f(arguments)),n?null:this,t),n?void 0:this}}var o=e(&quot;handle&quot;),a=e(4),f=e(5),c=e(&quot;ee&quot;).get(&quot;tracer&quot;),u=e(&quot;loader&quot;),s=NREUM;&quot;undefined&quot;==typeof window.newrelic&amp;&amp;(newrelic=s);var p=[&quot;setPageViewName&quot;,&quot;setCustomAttribute&quot;,&quot;setErrorHandler&quot;,&quot;finished&quot;,&quot;addToTrace&quot;,&quot;inlineHit&quot;,&quot;addRelease&quot;],l=&quot;api-&quot;,d=l+&quot;ixn-&quot;;a(p,function(e,n){s[n]=i(l+n,!0,&quot;api&quot;)}),s.addPageAction=i(l+&quot;addPageAction&quot;,!0),s.setCurrentRouteName=i(l+&quot;routeName&quot;,!0),n.exports=newrelic,s.interaction=function(){return(new r).get()};var m=r.prototype={createTracer:function(e,n){var t={},r=this,i=&quot;function&quot;==typeof n;return o(d+&quot;tracer&quot;,[u.now(),e,t],r),function(){if(c.emit((i?&quot;&quot;:&quot;no-&quot;)+&quot;fn-start&quot;,[u.now(),r,i],t),i)try{return n.apply(this,arguments)}catch(e){throw c.emit(&quot;fn-err&quot;,[arguments,this,e],t),e}finally{c.emit(&quot;fn-end&quot;,[u.now()],t)}}}};a(&quot;actionText,setName,setAttribute,save,ignore,onEnd,getContext,end,get&quot;.split(&quot;,&quot;),function(e,n){m[n]=i(d+n)}),newrelic.noticeError=function(e,n){&quot;string&quot;==typeof e&amp;&amp;(e=new Error(e)),o(&quot;err&quot;,[e,u.now(),!1,n])}},{}],2:[function(e,n,t){function r(e,n){var t=e.getEntries();t.forEach(function(e){&quot;first-paint&quot;===e.name?c(&quot;timing&quot;,[&quot;fp&quot;,Math.floor(e.startTime)]):&quot;first-contentful-paint&quot;===e.name&amp;&amp;c(&quot;timing&quot;,[&quot;fcp&quot;,Math.floor(e.startTime)])})}function i(e,n){var t=e.getEntries();t.length>0&amp;&amp;c(&quot;lcp&quot;,[t[t.length-1]])}function o(e){if(e instanceof s&amp;&amp;!l){var n,t=Math.round(e.timeStamp);n=t>1e12?Date.now()-t:u.now()-t,l=!0,c(&quot;timing&quot;,[&quot;fi&quot;,t,{type:e.type,fid:n}])}}if(!(&quot;init&quot;in NREUM&amp;&amp;&quot;page_view_timing&quot;in NREUM.init&amp;&amp;&quot;enabled&quot;in NREUM.init.page_view_timing&amp;&amp;NREUM.init.page_view_timing.enabled===!1)){var a,f,c=e(&quot;handle&quot;),u=e(&quot;loader&quot;),s=NREUM.o.EV;if(&quot;PerformanceObserver&quot;in window&amp;&amp;&quot;function&quot;==typeof window.PerformanceObserver){a=new PerformanceObserver(r),f=new PerformanceObserver(i);try{a.observe({entryTypes:[&quot;paint&quot;]}),f.observe({entryTypes:[&quot;largest-contentful-paint&quot;]})}catch(p){}}if(&quot;addEventListener&quot;in document){var l=!1,d=[&quot;click&quot;,&quot;keydown&quot;,&quot;mousedown&quot;,&quot;pointerdown&quot;,&quot;touchstart&quot;];d.forEach(function(e){document.addEventListener(e,o,!1)})}}},{}],3:[function(e,n,t){function r(e,n){if(!i)return!1;if(e!==i)return!1;if(!n)return!0;if(!o)return!1;for(var t=o.split(&quot;.&quot;),r=n.split(&quot;.&quot;),a=0;a&lt;r.length;a++)if(r[a]!==t[a])return!1;return!0}var i=null,o=null,a=/Version\/(\S+)\s+Safari/;if(navigator.userAgent){var f=navigator.userAgent,c=f.match(a);c&amp;&amp;f.indexOf(&quot;Chrome&quot;)===-1&amp;&amp;f.indexOf(&quot;Chromium&quot;)===-1&amp;&amp;(i=&quot;Safari&quot;,o=c[1])}n.exports={agent:i,version:o,match:r}},{}],4:[function(e,n,t){function r(e,n){var t=[],r=&quot;&quot;,o=0;for(r in e)i.call(e,r)&amp;&amp;(t[o]=n(r,e[r]),o+=1);return t}var i=Object.prototype.hasOwnProperty;n.exports=r},{}],5:[function(e,n,t){function r(e,n,t){n||(n=0),&quot;undefined&quot;==typeof t&amp;&amp;(t=e?e.length:0);for(var r=-1,i=t-n||0,o=Array(i&lt;0?0:i);++r&lt;i;)o[r]=e[n+r];return o}n.exports=r},{}],6:[function(e,n,t){n.exports={exists:&quot;undefined&quot;!=typeof window.performance&amp;&amp;window.performance.timing&amp;&amp;&quot;undefined&quot;!=typeof window.performance.timing.navigationStart}},{}],ee:[function(e,n,t){function r(){}function i(e){function n(e){return e&amp;&amp;e instanceof r?e:e?c(e,f,o):o()}function t(t,r,i,o){if(!l.aborted||o){e&amp;&amp;e(t,r,i);for(var a=n(i),f=v(t),c=f.length,u=0;u&lt;c;u++)f[u].apply(a,r);var p=s[y[t]];return p&amp;&amp;p.push([b,t,r,a]),a}}function d(e,n){h[e]=v(e).concat(n)}function m(e,n){var t=h[e];if(t)for(var r=0;r&lt;t.length;r++)t[r]===n&amp;&amp;t.splice(r,1)}function v(e){return h[e]||[]}function g(e){return p[e]=p[e]||i(t)}function w(e,n){u(e,function(e,t){n=n||&quot;feature&quot;,y[t]=n,n in s||(s[n]=[])})}var h={},y={},b={on:d,addEventListener:d,removeEventListener:m,emit:t,get:g,listeners:v,context:n,buffer:w,abort:a,aborted:!1};return b}function o(){return new r}function a(){(s.api||s.feature)&amp;&amp;(l.aborted=!0,s=l.backlog={})}var f=&quot;nr@context&quot;,c=e(&quot;gos&quot;),u=e(4),s={},p={},l=n.exports=i();l.backlog=s},{}],gos:[function(e,n,t){function r(e,n,t){if(i.call(e,n))return e[n];var r=t();if(Object.defineProperty&amp;&amp;Object.keys)try{return Object.defineProperty(e,n,{value:r,writable:!0,enumerable:!1}),r}catch(o){}return e[n]=r,r}var i=Object.prototype.hasOwnProperty;n.exports=r},{}],handle:[function(e,n,t){function r(e,n,t,r){i.buffer([e],r),i.emit(e,n,t)}var i=e(&quot;ee&quot;).get(&quot;handle&quot;);n.exports=r,r.ee=i},{}],id:[function(e,n,t){function r(e){var n=typeof e;return!e||&quot;object&quot;!==n&amp;&amp;&quot;function&quot;!==n?-1:e===window?0:a(e,o,function(){return i++})}var i=1,o=&quot;nr@id&quot;,a=e(&quot;gos&quot;);n.exports=r},{}],loader:[function(e,n,t){function r(){if(!x++){var e=E.info=NREUM.info,n=d.getElementsByTagName(&quot;script&quot;)[0];if(setTimeout(s.abort,3e4),!(e&amp;&amp;e.licenseKey&amp;&amp;e.applicationID&amp;&amp;n))return s.abort();u(y,function(n,t){e[n]||(e[n]=t)}),c(&quot;mark&quot;,[&quot;onload&quot;,a()+E.offset],null,&quot;api&quot;);var t=d.createElement(&quot;script&quot;);t.src=&quot;https://&quot;+e.agent,n.parentNode.insertBefore(t,n)}}function i(){&quot;complete&quot;===d.readyState&amp;&amp;o()}function o(){c(&quot;mark&quot;,[&quot;domContent&quot;,a()+E.offset],null,&quot;api&quot;)}function a(){return O.exists&amp;&amp;performance.now?Math.round(performance.now()):(f=Math.max((new Date).getTime(),f))-E.offset}var f=(new Date).getTime(),c=e(&quot;handle&quot;),u=e(4),s=e(&quot;ee&quot;),p=e(3),l=window,d=l.document,m=&quot;addEventListener&quot;,v=&quot;attachEvent&quot;,g=l.XMLHttpRequest,w=g&amp;&amp;g.prototype;NREUM.o={ST:setTimeout,SI:l.setImmediate,CT:clearTimeout,XHR:g,REQ:l.Request,EV:l.Event,PR:l.Promise,MO:l.MutationObserver};var h=&quot;&quot;+location,y={beacon:&quot;bam.nr-data.net&quot;,errorBeacon:&quot;bam.nr-data.net&quot;,agent:&quot;js-agent.newrelic.com/nr-1173.min.js&quot;},b=g&amp;&amp;w&amp;&amp;w[m]&amp;&amp;!/CriOS/.test(navigator.userAgent),E=n.exports={offset:f,now:a,origin:h,features:{},xhrWrappable:b,userAgent:p};e(1),e(2),d[m]?(d[m](&quot;DOMContentLoaded&quot;,o,!1),l[m](&quot;load&quot;,r,!1)):(d[v](&quot;onreadystatechange&quot;,i),l[v](&quot;onload&quot;,r)),c(&quot;mark&quot;,[&quot;firstbyte&quot;,f],null,&quot;api&quot;);var x=0,O=e(6)},{}],&quot;wrap-function&quot;:[function(e,n,t){function r(e){return!(e&amp;&amp;e instanceof Function&amp;&amp;e.apply&amp;&amp;!e[a])}var i=e(&quot;ee&quot;),o=e(5),a=&quot;nr@original&quot;,f=Object.prototype.hasOwnProperty,c=!1;n.exports=function(e,n){function t(e,n,t,i){function nrWrapper(){var r,a,f,c;try{a=this,r=o(arguments),f=&quot;function&quot;==typeof t?t(r,a):t||{}}catch(u){l([u,&quot;&quot;,[r,a,i],f])}s(n+&quot;start&quot;,[r,a,i],f);try{return c=e.apply(a,r)}catch(p){throw s(n+&quot;err&quot;,[r,a,p],f),p}finally{s(n+&quot;end&quot;,[r,a,c],f)}}return r(e)?e:(n||(n=&quot;&quot;),nrWrapper[a]=e,p(e,nrWrapper),nrWrapper)}function u(e,n,i,o){i||(i=&quot;&quot;);var a,f,c,u=&quot;-&quot;===i.charAt(0);for(c=0;c&lt;n.length;c++)f=n[c],a=e[f],r(a)||(e[f]=t(a,u?f+i:i,o,f))}function s(t,r,i){if(!c||n){var o=c;c=!0;try{e.emit(t,r,i,n)}catch(a){l([a,t,r,i])}c=o}}function p(e,n){if(Object.defineProperty&amp;&amp;Object.keys)try{var t=Object.keys(e);return t.forEach(function(t){Object.defineProperty(n,t,{get:function(){return e[t]},set:function(n){return e[t]=n,n}})}),n}catch(r){l([r])}for(var i in e)f.call(e,i)&amp;&amp;(n[i]=e[i]);return n}function l(n){try{e.emit(&quot;internal-error&quot;,n)}catch(t){}}return e||(e=i),t.inPlace=u,t.flag=a,t}},{}]},{},[&quot;loader&quot;]);
    Qiscus Multichannel Chat
    
     
	
	
	
	
	
	
	
    
    
    (function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
    new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
    j=d.createElement(s),dl=l!='dataLayer'?'&amp;l='+l:'';j.async=true;j.src=
    'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
    })(window,document,'script','dataLayer','GTM-WKTB94R');
    
    
    (function(h,o,t,j,a,r){
    h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)};
    h._hjSettings={hjid:1634887,hjsv:6};
    a=o.getElementsByTagName('head')[0];
    r=o.createElement('script');r.async=1;
    r.src=t+h._hjSettings.hjid+j+h._hjSettings.hjsv;
    a.appendChild(r);
    })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');
    
html.swal2-shown:not(.swal2-no-backdrop):not(.swal2-toast-shown),
body.swal2-shown:not(.swal2-no-backdrop):not(.swal2-toast-shown) {
  overflow-y: hidden; }

body.swal2-toast-shown.swal2-has-input > .swal2-container > .swal2-toast {
  -webkit-box-orient: vertical;
  -webkit-box-direction: normal;
      -ms-flex-direction: column;
          flex-direction: column; }
  body.swal2-toast-shown.swal2-has-input > .swal2-container > .swal2-toast .swal2-icon {
    margin: 0 0 15px; }
  body.swal2-toast-shown.swal2-has-input > .swal2-container > .swal2-toast .swal2-buttonswrapper {
    -webkit-box-flex: 1;
        -ms-flex: 1;
            flex: 1;
    -ms-flex-item-align: stretch;
        align-self: stretch;
    -webkit-box-pack: end;
        -ms-flex-pack: end;
            justify-content: flex-end; }
  body.swal2-toast-shown.swal2-has-input > .swal2-container > .swal2-toast .swal2-loading {
    -webkit-box-pack: center;
        -ms-flex-pack: center;
            justify-content: center; }
  body.swal2-toast-shown.swal2-has-input > .swal2-container > .swal2-toast .swal2-input {
    height: 32px;
    font-size: 14px;
    margin: 5px auto; }

body.swal2-toast-shown > .swal2-container {
  position: fixed;
  background-color: transparent; }
  body.swal2-toast-shown > .swal2-container.swal2-shown {
    background-color: transparent; }
  body.swal2-toast-shown > .swal2-container.swal2-top {
    top: 0;
    left: 50%;
    bottom: auto;
    right: auto;
    -webkit-transform: translateX(-50%);
            transform: translateX(-50%); }
  body.swal2-toast-shown > .swal2-container.swal2-top-right {
    top: 0;
    left: auto;
    bottom: auto;
    right: 0; }
  body.swal2-toast-shown > .swal2-container.swal2-top-left {
    top: 0;
    left: 0;
    bottom: auto;
    right: auto; }
  body.swal2-toast-shown > .swal2-container.swal2-center-left {
    top: 50%;
    left: 0;
    bottom: auto;
    right: auto;
    -webkit-transform: translateY(-50%);
            transform: translateY(-50%); }
  body.swal2-toast-shown > .swal2-container.swal2-center {
    top: 50%;
    left: 50%;
    bottom: auto;
    right: auto;
    -webkit-transform: translate(-50%, -50%);
            transform: translate(-50%, -50%); }
  body.swal2-toast-shown > .swal2-container.swal2-center-right {
    top: 50%;
    left: auto;
    bottom: auto;
    right: 0;
    -webkit-transform: translateY(-50%);
            transform: translateY(-50%); }
  body.swal2-toast-shown > .swal2-container.swal2-bottom-left {
    top: auto;
    left: 0;
    bottom: 0;
    right: auto; }
  body.swal2-toast-shown > .swal2-container.swal2-bottom {
    top: auto;
    left: 50%;
    bottom: 0;
    right: auto;
    -webkit-transform: translateX(-50%);
            transform: translateX(-50%); }
  body.swal2-toast-shown > .swal2-container.swal2-bottom-right {
    top: auto;
    left: auto;
    bottom: 0;
    right: 0; }

body.swal2-iosfix {
  position: fixed;
  left: 0;
  right: 0; }

body.swal2-no-backdrop > .swal2-shown {
  top: auto;
  bottom: auto;
  left: auto;
  right: auto;
  background-color: transparent; }
  body.swal2-no-backdrop > .swal2-shown > .swal2-modal {
    -webkit-box-shadow: 0 0 10px rgba(0, 0, 0, 0.4);
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.4); }
  body.swal2-no-backdrop > .swal2-shown.swal2-top {
    top: 0;
    left: 50%;
    -webkit-transform: translateX(-50%);
            transform: translateX(-50%); }
  body.swal2-no-backdrop > .swal2-shown.swal2-top-left {
    top: 0;
    left: 0; }
  body.swal2-no-backdrop > .swal2-shown.swal2-top-right {
    top: 0;
    right: 0; }
  body.swal2-no-backdrop > .swal2-shown.swal2-center {
    top: 50%;
    left: 50%;
    -webkit-transform: translate(-50%, -50%);
            transform: translate(-50%, -50%); }
  body.swal2-no-backdrop > .swal2-shown.swal2-center-left {
    top: 50%;
    left: 0;
    -webkit-transform: translateY(-50%);
            transform: translateY(-50%); }
  body.swal2-no-backdrop > .swal2-shown.swal2-center-right {
    top: 50%;
    right: 0;
    -webkit-transform: translateY(-50%);
            transform: translateY(-50%); }
  body.swal2-no-backdrop > .swal2-shown.swal2-bottom {
    bottom: 0;
    left: 50%;
    -webkit-transform: translateX(-50%);
            transform: translateX(-50%); }
  body.swal2-no-backdrop > .swal2-shown.swal2-bottom-left {
    bottom: 0;
    left: 0; }
  body.swal2-no-backdrop > .swal2-shown.swal2-bottom-right {
    bottom: 0;
    right: 0; }

.swal2-container {
  display: -webkit-box;
  display: -ms-flexbox;
  display: flex;
  -webkit-box-orient: horizontal;
  -webkit-box-direction: normal;
      -ms-flex-direction: row;
          flex-direction: row;
  -webkit-box-align: center;
      -ms-flex-align: center;
          align-items: center;
  -webkit-box-pack: center;
      -ms-flex-pack: center;
          justify-content: center;
  position: fixed;
  padding: 10px;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: transparent;
  z-index: 1060; }
  .swal2-container.swal2-top {
    -webkit-box-align: start;
        -ms-flex-align: start;
            align-items: flex-start; }
  .swal2-container.swal2-top-left {
    -webkit-box-align: start;
        -ms-flex-align: start;
            align-items: flex-start;
    -webkit-box-pack: start;
        -ms-flex-pack: start;
            justify-content: flex-start; }
  .swal2-container.swal2-top-right {
    -webkit-box-align: start;
        -ms-flex-align: start;
            align-items: flex-start;
    -webkit-box-pack: end;
        -ms-flex-pack: end;
            justify-content: flex-end; }
  .swal2-container.swal2-center {
    -webkit-box-align: center;
        -ms-flex-align: center;
            align-items: center; }
  .swal2-container.swal2-center-left {
    -webkit-box-align: center;
        -ms-flex-align: center;
            align-items: center;
    -webkit-box-pack: start;
        -ms-flex-pack: start;
            justify-content: flex-start; }
  .swal2-container.swal2-center-right {
    -webkit-box-align: center;
        -ms-flex-align: center;
            align-items: center;
    -webkit-box-pack: end;
        -ms-flex-pack: end;
            justify-content: flex-end; }
  .swal2-container.swal2-bottom {
    -webkit-box-align: end;
        -ms-flex-align: end;
            align-items: flex-end; }
  .swal2-container.swal2-bottom-left {
    -webkit-box-align: end;
        -ms-flex-align: end;
            align-items: flex-end;
    -webkit-box-pack: start;
        -ms-flex-pack: start;
            justify-content: flex-start; }
  .swal2-container.swal2-bottom-right {
    -webkit-box-align: end;
        -ms-flex-align: end;
            align-items: flex-end;
    -webkit-box-pack: end;
        -ms-flex-pack: end;
            justify-content: flex-end; }
  .swal2-container.swal2-grow-fullscreen > .swal2-modal {
    display: -webkit-box !important;
    display: -ms-flexbox !important;
    display: flex !important;
    -webkit-box-flex: 1;
        -ms-flex: 1;
            flex: 1;
    -ms-flex-item-align: stretch;
        align-self: stretch;
    -webkit-box-pack: center;
        -ms-flex-pack: center;
            justify-content: center; }
  .swal2-container.swal2-grow-row > .swal2-modal {
    display: -webkit-box !important;
    display: -ms-flexbox !important;
    display: flex !important;
    -webkit-box-flex: 1;
        -ms-flex: 1;
            flex: 1;
    -ms-flex-line-pack: center;
        align-content: center;
    -webkit-box-pack: center;
        -ms-flex-pack: center;
            justify-content: center; }
  .swal2-container.swal2-grow-column {
    -webkit-box-flex: 1;
        -ms-flex: 1;
            flex: 1;
    -webkit-box-orient: vertical;
    -webkit-box-direction: normal;
        -ms-flex-direction: column;
            flex-direction: column; }
    .swal2-container.swal2-grow-column.swal2-top, .swal2-container.swal2-grow-column.swal2-center, .swal2-container.swal2-grow-column.swal2-bottom {
      -webkit-box-align: center;
          -ms-flex-align: center;
              align-items: center; }
    .swal2-container.swal2-grow-column.swal2-top-left, .swal2-container.swal2-grow-column.swal2-center-left, .swal2-container.swal2-grow-column.swal2-bottom-left {
      -webkit-box-align: start;
          -ms-flex-align: start;
              align-items: flex-start; }
    .swal2-container.swal2-grow-column.swal2-top-right, .swal2-container.swal2-grow-column.swal2-center-right, .swal2-container.swal2-grow-column.swal2-bottom-right {
      -webkit-box-align: end;
          -ms-flex-align: end;
              align-items: flex-end; }
    .swal2-container.swal2-grow-column > .swal2-modal {
      display: -webkit-box !important;
      display: -ms-flexbox !important;
      display: flex !important;
      -webkit-box-flex: 1;
          -ms-flex: 1;
              flex: 1;
      -ms-flex-line-pack: center;
          align-content: center;
      -webkit-box-pack: center;
          -ms-flex-pack: center;
              justify-content: center; }
  .swal2-container:not(.swal2-top):not(.swal2-top-left):not(.swal2-top-right):not(.swal2-center-left):not(.swal2-center-right):not(.swal2-bottom):not(.swal2-bottom-left):not(.swal2-bottom-right) > .swal2-modal {
    margin: auto; }
  @media all and (-ms-high-contrast: none), (-ms-high-contrast: active) {
    .swal2-container .swal2-modal {
      margin: 0 !important; } }
  .swal2-container.swal2-fade {
    -webkit-transition: background-color .1s;
    transition: background-color .1s; }
  .swal2-container.swal2-shown {
    background-color: rgba(0, 0, 0, 0.4); }

.swal2-popup {
  -webkit-box-orient: vertical;
  -webkit-box-direction: normal;
      -ms-flex-direction: column;
          flex-direction: column;
  background-color: #fff;
  font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
  border-radius: 5px;
  -webkit-box-sizing: border-box;
          box-sizing: border-box;
  text-align: center;
  overflow-x: hidden;
  overflow-y: auto;
  display: none;
  position: relative;
  max-width: 100%; }
  .swal2-popup.swal2-toast {
    width: 300px;
    padding: 0 15px;
    -webkit-box-orient: horizontal;
    -webkit-box-direction: normal;
        -ms-flex-direction: row;
            flex-direction: row;
    -webkit-box-align: center;
        -ms-flex-align: center;
            align-items: center;
    overflow-y: hidden;
    -webkit-box-shadow: 0 0 10px #d9d9d9;
            box-shadow: 0 0 10px #d9d9d9; }
    .swal2-popup.swal2-toast .swal2-title {
      max-width: 300px;
      font-size: 16px;
      text-align: left; }
    .swal2-popup.swal2-toast .swal2-content {
      font-size: 14px;
      text-align: left; }
    .swal2-popup.swal2-toast .swal2-icon {
      width: 32px;
      min-width: 32px;
      height: 32px;
      margin: 0 15px 0 0; }
      .swal2-popup.swal2-toast .swal2-icon.swal2-success .swal2-success-ring {
        width: 32px;
        height: 32px; }
      .swal2-popup.swal2-toast .swal2-icon.swal2-info, .swal2-popup.swal2-toast .swal2-icon.swal2-warning, .swal2-popup.swal2-toast .swal2-icon.swal2-question {
        font-size: 26px;
        line-height: 32px; }
      .swal2-popup.swal2-toast .swal2-icon.swal2-error [class^='swal2-x-mark-line'] {
        top: 14px;
        width: 22px; }
        .swal2-popup.swal2-toast .swal2-icon.swal2-error [class^='swal2-x-mark-line'][class$='left'] {
          left: 5px; }
        .swal2-popup.swal2-toast .swal2-icon.swal2-error [class^='swal2-x-mark-line'][class$='right'] {
          right: 5px; }
    .swal2-popup.swal2-toast .swal2-buttonswrapper {
      margin: 0 0 0 5px; }
    .swal2-popup.swal2-toast .swal2-styled {
      margin: 0 0 0 5px;
      padding: 5px 10px; }
      .swal2-popup.swal2-toast .swal2-styled:focus {
        -webkit-box-shadow: 0 0 0 1px #fff, 0 0 0 2px rgba(50, 100, 150, 0.4);
                box-shadow: 0 0 0 1px #fff, 0 0 0 2px rgba(50, 100, 150, 0.4); }
    .swal2-popup.swal2-toast .swal2-validationerror {
      width: 100%;
      margin: 5px -20px; }
    .swal2-popup.swal2-toast .swal2-success {
      border-color: #a5dc86; }
      .swal2-popup.swal2-toast .swal2-success [class^='swal2-success-circular-line'] {
        border-radius: 50%;
        position: absolute;
        width: 32px;
        height: 64px;
        -webkit-transform: rotate(45deg);
                transform: rotate(45deg); }
        .swal2-popup.swal2-toast .swal2-success [class^='swal2-success-circular-line'][class$='left'] {
          border-radius: 64px 0 0 64px;
          top: -4px;
          left: -15px;
          -webkit-transform: rotate(-45deg);
                  transform: rotate(-45deg);
          -webkit-transform-origin: 32px 32px;
                  transform-origin: 32px 32px; }
        .swal2-popup.swal2-toast .swal2-success [class^='swal2-success-circular-line'][class$='right'] {
          border-radius: 0 64px 64px 0;
          top: -5px;
          left: 14px;
          -webkit-transform-origin: 0 32px;
                  transform-origin: 0 32px; }
      .swal2-popup.swal2-toast .swal2-success .swal2-success-ring {
        width: 32px;
        height: 32px; }
      .swal2-popup.swal2-toast .swal2-success .swal2-success-fix {
        width: 7px;
        height: 90px;
        left: 28px;
        top: 8px; }
      .swal2-popup.swal2-toast .swal2-success [class^='swal2-success-line'] {
        height: 5px; }
        .swal2-popup.swal2-toast .swal2-success [class^='swal2-success-line'][class$='tip'] {
          width: 12px;
          left: 3px;
          top: 18px; }
        .swal2-popup.swal2-toast .swal2-success [class^='swal2-success-line'][class$='long'] {
          width: 22px;
          right: 3px;
          top: 15px; }
    .swal2-popup.swal2-toast .swal2-animate-success-line-tip {
      -webkit-animation: animate-toast-success-tip .75s;
              animation: animate-toast-success-tip .75s; }
    .swal2-popup.swal2-toast .swal2-animate-success-line-long {
      -webkit-animation: animate-toast-success-long .75s;
              animation: animate-toast-success-long .75s; }
  .swal2-popup:focus {
    outline: none; }
  .swal2-popup.swal2-loading {
    overflow-y: hidden; }
  .swal2-popup .swal2-title {
    color: #595959;
    font-size: 30px;
    text-align: center;
    font-weight: 600;
    text-transform: none;
    position: relative;
    margin: 0 0 .4em;
    padding: 0;
    display: block;
    word-wrap: break-word; }
  .swal2-popup .swal2-buttonswrapper {
    -webkit-box-align: center;
        -ms-flex-align: center;
            align-items: center;
    -webkit-box-pack: center;
        -ms-flex-pack: center;
            justify-content: center;
    margin-top: 15px; }
    .swal2-popup .swal2-buttonswrapper:not(.swal2-loading) .swal2-styled[disabled] {
      opacity: .4;
      cursor: no-drop; }
    .swal2-popup .swal2-buttonswrapper.swal2-loading .swal2-styled.swal2-confirm {
      -webkit-box-sizing: border-box;
              box-sizing: border-box;
      border: 4px solid transparent;
      border-color: transparent;
      width: 40px;
      height: 40px;
      padding: 0;
      margin: 7.5px;
      vertical-align: top;
      background-color: transparent !important;
      color: transparent;
      cursor: default;
      border-radius: 100%;
      -webkit-animation: rotate-loading 1.5s linear 0s infinite normal;
              animation: rotate-loading 1.5s linear 0s infinite normal;
      -webkit-user-select: none;
         -moz-user-select: none;
          -ms-user-select: none;
              user-select: none; }
    .swal2-popup .swal2-buttonswrapper.swal2-loading .swal2-styled.swal2-cancel {
      margin-left: 30px;
      margin-right: 30px; }
    .swal2-popup .swal2-buttonswrapper.swal2-loading :not(.swal2-styled).swal2-confirm::after {
      display: inline-block;
      content: '';
      margin-left: 5px;
      vertical-align: -1px;
      height: 15px;
      width: 15px;
      border: 3px solid #999999;
      -webkit-box-shadow: 1px 1px 1px #fff;
              box-shadow: 1px 1px 1px #fff;
      border-right-color: transparent;
      border-radius: 50%;
      -webkit-animation: rotate-loading 1.5s linear 0s infinite normal;
              animation: rotate-loading 1.5s linear 0s infinite normal; }
  .swal2-popup .swal2-styled {
    border: 0;
    border-radius: 3px;
    -webkit-box-shadow: none;
            box-shadow: none;
    color: #fff;
    cursor: pointer;
    font-size: 17px;
    font-weight: 500;
    margin: 15px 5px 0;
    padding: 10px 32px; }
    .swal2-popup .swal2-styled:focus {
      outline: none;
      -webkit-box-shadow: 0 0 0 2px #fff, 0 0 0 4px rgba(50, 100, 150, 0.4);
              box-shadow: 0 0 0 2px #fff, 0 0 0 4px rgba(50, 100, 150, 0.4); }
  .swal2-popup .swal2-image {
    margin: 20px auto;
    max-width: 100%; }
  .swal2-popup .swal2-close {
    background: transparent;
    border: 0;
    margin: 0;
    padding: 0;
    width: 38px;
    height: 40px;
    font-size: 36px;
    line-height: 40px;
    font-family: serif;
    position: absolute;
    top: 5px;
    right: 8px;
    cursor: pointer;
    color: #cccccc;
    -webkit-transition: color .1s ease;
    transition: color .1s ease; }
    .swal2-popup .swal2-close:hover {
      color: #d55; }
  .swal2-popup > .swal2-input,
  .swal2-popup > .swal2-file,
  .swal2-popup > .swal2-textarea,
  .swal2-popup > .swal2-select,
  .swal2-popup > .swal2-radio,
  .swal2-popup > .swal2-checkbox {
    display: none; }
  .swal2-popup .swal2-content {
    font-size: 18px;
    text-align: center;
    font-weight: 300;
    position: relative;
    float: none;
    margin: 0;
    padding: 0;
    line-height: normal;
    color: #545454;
    word-wrap: break-word; }
  .swal2-popup .swal2-input,
  .swal2-popup .swal2-file,
  .swal2-popup .swal2-textarea,
  .swal2-popup .swal2-select,
  .swal2-popup .swal2-radio,
  .swal2-popup .swal2-checkbox {
    margin: 20px auto; }
  .swal2-popup .swal2-input,
  .swal2-popup .swal2-file,
  .swal2-popup .swal2-textarea {
    width: 100%;
    -webkit-box-sizing: border-box;
            box-sizing: border-box;
    font-size: 18px;
    border-radius: 3px;
    border: 1px solid #d9d9d9;
    -webkit-box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.06);
            box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.06);
    -webkit-transition: border-color .3s, -webkit-box-shadow .3s;
    transition: border-color .3s, -webkit-box-shadow .3s;
    transition: border-color .3s, box-shadow .3s;
    transition: border-color .3s, box-shadow .3s, -webkit-box-shadow .3s; }
    .swal2-popup .swal2-input.swal2-inputerror,
    .swal2-popup .swal2-file.swal2-inputerror,
    .swal2-popup .swal2-textarea.swal2-inputerror {
      border-color: #f27474 !important;
      -webkit-box-shadow: 0 0 2px #f27474 !important;
              box-shadow: 0 0 2px #f27474 !important; }
    .swal2-popup .swal2-input:focus,
    .swal2-popup .swal2-file:focus,
    .swal2-popup .swal2-textarea:focus {
      outline: none;
      border: 1px solid #b4dbed;
      -webkit-box-shadow: 0 0 3px #c4e6f5;
              box-shadow: 0 0 3px #c4e6f5; }
    .swal2-popup .swal2-input::-webkit-input-placeholder,
    .swal2-popup .swal2-file::-webkit-input-placeholder,
    .swal2-popup .swal2-textarea::-webkit-input-placeholder {
      color: #cccccc; }
    .swal2-popup .swal2-input:-ms-input-placeholder,
    .swal2-popup .swal2-file:-ms-input-placeholder,
    .swal2-popup .swal2-textarea:-ms-input-placeholder {
      color: #cccccc; }
    .swal2-popup .swal2-input::-ms-input-placeholder,
    .swal2-popup .swal2-file::-ms-input-placeholder,
    .swal2-popup .swal2-textarea::-ms-input-placeholder {
      color: #cccccc; }
    .swal2-popup .swal2-input::placeholder,
    .swal2-popup .swal2-file::placeholder,
    .swal2-popup .swal2-textarea::placeholder {
      color: #cccccc; }
  .swal2-popup .swal2-range input {
    float: left;
    width: 80%; }
  .swal2-popup .swal2-range output {
    float: right;
    width: 20%;
    font-size: 20px;
    font-weight: 600;
    text-align: center; }
  .swal2-popup .swal2-range input,
  .swal2-popup .swal2-range output {
    height: 43px;
    line-height: 43px;
    vertical-align: middle;
    margin: 20px auto;
    padding: 0; }
  .swal2-popup .swal2-input {
    height: 43px;
    padding: 0 12px; }
    .swal2-popup .swal2-input[type='number'] {
      max-width: 150px; }
  .swal2-popup .swal2-file {
    font-size: 20px; }
  .swal2-popup .swal2-textarea {
    height: 108px;
    padding: 12px; }
  .swal2-popup .swal2-select {
    color: #545454;
    font-size: inherit;
    padding: 5px 10px;
    min-width: 40%;
    max-width: 100%; }
  .swal2-popup .swal2-radio {
    border: 0; }
    .swal2-popup .swal2-radio label:not(:first-child) {
      margin-left: 20px; }
    .swal2-popup .swal2-radio input,
    .swal2-popup .swal2-radio span {
      vertical-align: middle; }
    .swal2-popup .swal2-radio input {
      margin: 0 3px 0 0; }
  .swal2-popup .swal2-checkbox {
    color: #545454; }
    .swal2-popup .swal2-checkbox input,
    .swal2-popup .swal2-checkbox span {
      vertical-align: middle; }
  .swal2-popup .swal2-validationerror {
    background-color: #f0f0f0;
    margin: 0 -20px;
    overflow: hidden;
    padding: 10px;
    color: gray;
    font-size: 16px;
    font-weight: 300;
    display: none; }
    .swal2-popup .swal2-validationerror::before {
      content: '!';
      display: inline-block;
      width: 24px;
      height: 24px;
      border-radius: 50%;
      background-color: #ea7d7d;
      color: #fff;
      line-height: 24px;
      text-align: center;
      margin-right: 10px; }

@supports (-ms-accelerator: true) {
  .swal2-range input {
    width: 100% !important; }
  .swal2-range output {
    display: none; } }

@media all and (-ms-high-contrast: none), (-ms-high-contrast: active) {
  .swal2-range input {
    width: 100% !important; }
  .swal2-range output {
    display: none; } }

.swal2-icon {
  width: 80px;
  height: 80px;
  border: 4px solid transparent;
  border-radius: 50%;
  margin: 20px auto 30px;
  padding: 0;
  position: relative;
  -webkit-box-sizing: content-box;
          box-sizing: content-box;
  cursor: default;
  -webkit-user-select: none;
     -moz-user-select: none;
      -ms-user-select: none;
          user-select: none; }
  .swal2-icon.swal2-error {
    border-color: #f27474; }
    .swal2-icon.swal2-error .swal2-x-mark {
      position: relative;
      display: block; }
    .swal2-icon.swal2-error [class^='swal2-x-mark-line'] {
      position: absolute;
      height: 5px;
      width: 47px;
      background-color: #f27474;
      display: block;
      top: 37px;
      border-radius: 2px; }
      .swal2-icon.swal2-error [class^='swal2-x-mark-line'][class$='left'] {
        -webkit-transform: rotate(45deg);
                transform: rotate(45deg);
        left: 17px; }
      .swal2-icon.swal2-error [class^='swal2-x-mark-line'][class$='right'] {
        -webkit-transform: rotate(-45deg);
                transform: rotate(-45deg);
        right: 16px; }
  .swal2-icon.swal2-warning {
    font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
    color: #f8bb86;
    border-color: #facea8;
    font-size: 60px;
    line-height: 80px;
    text-align: center; }
  .swal2-icon.swal2-info {
    font-family: 'Open Sans', sans-serif;
    color: #3fc3ee;
    border-color: #9de0f6;
    font-size: 60px;
    line-height: 80px;
    text-align: center; }
  .swal2-icon.swal2-question {
    font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
    color: #87adbd;
    border-color: #c9dae1;
    font-size: 60px;
    line-height: 80px;
    text-align: center; }
  .swal2-icon.swal2-success {
    border-color: #a5dc86; }
    .swal2-icon.swal2-success [class^='swal2-success-circular-line'] {
      border-radius: 50%;
      position: absolute;
      width: 60px;
      height: 120px;
      -webkit-transform: rotate(45deg);
              transform: rotate(45deg); }
      .swal2-icon.swal2-success [class^='swal2-success-circular-line'][class$='left'] {
        border-radius: 120px 0 0 120px;
        top: -7px;
        left: -33px;
        -webkit-transform: rotate(-45deg);
                transform: rotate(-45deg);
        -webkit-transform-origin: 60px 60px;
                transform-origin: 60px 60px; }
      .swal2-icon.swal2-success [class^='swal2-success-circular-line'][class$='right'] {
        border-radius: 0 120px 120px 0;
        top: -11px;
        left: 30px;
        -webkit-transform: rotate(-45deg);
                transform: rotate(-45deg);
        -webkit-transform-origin: 0 60px;
                transform-origin: 0 60px; }
    .swal2-icon.swal2-success .swal2-success-ring {
      width: 80px;
      height: 80px;
      border: 4px solid rgba(165, 220, 134, 0.2);
      border-radius: 50%;
      -webkit-box-sizing: content-box;
              box-sizing: content-box;
      position: absolute;
      left: -4px;
      top: -4px;
      z-index: 2; }
    .swal2-icon.swal2-success .swal2-success-fix {
      width: 7px;
      height: 90px;
      position: absolute;
      left: 28px;
      top: 8px;
      z-index: 1;
      -webkit-transform: rotate(-45deg);
              transform: rotate(-45deg); }
    .swal2-icon.swal2-success [class^='swal2-success-line'] {
      height: 5px;
      background-color: #a5dc86;
      display: block;
      border-radius: 2px;
      position: absolute;
      z-index: 2; }
      .swal2-icon.swal2-success [class^='swal2-success-line'][class$='tip'] {
        width: 25px;
        left: 14px;
        top: 46px;
        -webkit-transform: rotate(45deg);
                transform: rotate(45deg); }
      .swal2-icon.swal2-success [class^='swal2-success-line'][class$='long'] {
        width: 47px;
        right: 8px;
        top: 38px;
        -webkit-transform: rotate(-45deg);
                transform: rotate(-45deg); }

.swal2-progresssteps {
  font-weight: 600;
  margin: 0 0 20px;
  padding: 0; }
  .swal2-progresssteps li {
    display: inline-block;
    position: relative; }
  .swal2-progresssteps .swal2-progresscircle {
    background: #3085d6;
    border-radius: 2em;
    color: #fff;
    height: 2em;
    line-height: 2em;
    text-align: center;
    width: 2em;
    z-index: 20; }
    .swal2-progresssteps .swal2-progresscircle:first-child {
      margin-left: 0; }
    .swal2-progresssteps .swal2-progresscircle:last-child {
      margin-right: 0; }
    .swal2-progresssteps .swal2-progresscircle.swal2-activeprogressstep {
      background: #3085d6; }
      .swal2-progresssteps .swal2-progresscircle.swal2-activeprogressstep ~ .swal2-progresscircle {
        background: #add8e6; }
      .swal2-progresssteps .swal2-progresscircle.swal2-activeprogressstep ~ .swal2-progressline {
        background: #add8e6; }
  .swal2-progresssteps .swal2-progressline {
    background: #3085d6;
    height: .4em;
    margin: 0 -1px;
    z-index: 10; }

[class^='swal2'] {
  -webkit-tap-highlight-color: transparent; }

@-webkit-keyframes showSweetToast {
  0% {
    -webkit-transform: translateY(-10px) rotateZ(2deg);
            transform: translateY(-10px) rotateZ(2deg);
    opacity: 0; }
  33% {
    -webkit-transform: translateY(0) rotateZ(-2deg);
            transform: translateY(0) rotateZ(-2deg);
    opacity: .5; }
  66% {
    -webkit-transform: translateY(5px) rotateZ(2deg);
            transform: translateY(5px) rotateZ(2deg);
    opacity: .7; }
  100% {
    -webkit-transform: translateY(0) rotateZ(0);
            transform: translateY(0) rotateZ(0);
    opacity: 1; } }

@keyframes showSweetToast {
  0% {
    -webkit-transform: translateY(-10px) rotateZ(2deg);
            transform: translateY(-10px) rotateZ(2deg);
    opacity: 0; }
  33% {
    -webkit-transform: translateY(0) rotateZ(-2deg);
            transform: translateY(0) rotateZ(-2deg);
    opacity: .5; }
  66% {
    -webkit-transform: translateY(5px) rotateZ(2deg);
            transform: translateY(5px) rotateZ(2deg);
    opacity: .7; }
  100% {
    -webkit-transform: translateY(0) rotateZ(0);
            transform: translateY(0) rotateZ(0);
    opacity: 1; } }

@-webkit-keyframes hideSweetToast {
  0% {
    opacity: 1; }
  33% {
    opacity: .5; }
  100% {
    -webkit-transform: rotateZ(1deg);
            transform: rotateZ(1deg);
    opacity: 0; } }

@keyframes hideSweetToast {
  0% {
    opacity: 1; }
  33% {
    opacity: .5; }
  100% {
    -webkit-transform: rotateZ(1deg);
            transform: rotateZ(1deg);
    opacity: 0; } }

@-webkit-keyframes showSweetAlert {
  0% {
    -webkit-transform: scale(0.7);
            transform: scale(0.7); }
  45% {
    -webkit-transform: scale(1.05);
            transform: scale(1.05); }
  80% {
    -webkit-transform: scale(0.95);
            transform: scale(0.95); }
  100% {
    -webkit-transform: scale(1);
            transform: scale(1); } }

@keyframes showSweetAlert {
  0% {
    -webkit-transform: scale(0.7);
            transform: scale(0.7); }
  45% {
    -webkit-transform: scale(1.05);
            transform: scale(1.05); }
  80% {
    -webkit-transform: scale(0.95);
            transform: scale(0.95); }
  100% {
    -webkit-transform: scale(1);
            transform: scale(1); } }

@-webkit-keyframes hideSweetAlert {
  0% {
    -webkit-transform: scale(1);
            transform: scale(1);
    opacity: 1; }
  100% {
    -webkit-transform: scale(0.5);
            transform: scale(0.5);
    opacity: 0; } }

@keyframes hideSweetAlert {
  0% {
    -webkit-transform: scale(1);
            transform: scale(1);
    opacity: 1; }
  100% {
    -webkit-transform: scale(0.5);
            transform: scale(0.5);
    opacity: 0; } }

.swal2-show {
  -webkit-animation: showSweetAlert .3s;
          animation: showSweetAlert .3s; }
  .swal2-show.swal2-toast {
    -webkit-animation: showSweetToast .5s;
            animation: showSweetToast .5s; }
  .swal2-show.swal2-noanimation {
    -webkit-animation: none;
            animation: none; }

.swal2-hide {
  -webkit-animation: hideSweetAlert .15s forwards;
          animation: hideSweetAlert .15s forwards; }
  .swal2-hide.swal2-toast {
    -webkit-animation: hideSweetToast .2s forwards;
            animation: hideSweetToast .2s forwards; }
  .swal2-hide.swal2-noanimation {
    -webkit-animation: none;
            animation: none; }

[dir='rtl'] .swal2-close {
  left: 8px;
  right: auto; }

@-webkit-keyframes animate-success-tip {
  0% {
    width: 0;
    left: 1px;
    top: 19px; }
  54% {
    width: 0;
    left: 1px;
    top: 19px; }
  70% {
    width: 50px;
    left: -8px;
    top: 37px; }
  84% {
    width: 17px;
    left: 21px;
    top: 48px; }
  100% {
    width: 25px;
    left: 14px;
    top: 45px; } }

@keyframes animate-success-tip {
  0% {
    width: 0;
    left: 1px;
    top: 19px; }
  54% {
    width: 0;
    left: 1px;
    top: 19px; }
  70% {
    width: 50px;
    left: -8px;
    top: 37px; }
  84% {
    width: 17px;
    left: 21px;
    top: 48px; }
  100% {
    width: 25px;
    left: 14px;
    top: 45px; } }

@-webkit-keyframes animate-success-long {
  0% {
    width: 0;
    right: 46px;
    top: 54px; }
  65% {
    width: 0;
    right: 46px;
    top: 54px; }
  84% {
    width: 55px;
    right: 0;
    top: 35px; }
  100% {
    width: 47px;
    right: 8px;
    top: 38px; } }

@keyframes animate-success-long {
  0% {
    width: 0;
    right: 46px;
    top: 54px; }
  65% {
    width: 0;
    right: 46px;
    top: 54px; }
  84% {
    width: 55px;
    right: 0;
    top: 35px; }
  100% {
    width: 47px;
    right: 8px;
    top: 38px; } }

@-webkit-keyframes animate-toast-success-tip {
  0% {
    width: 0;
    left: 1px;
    top: 9px; }
  54% {
    width: 0;
    left: 1px;
    top: 9px; }
  70% {
    width: 24px;
    left: -4px;
    top: 17px; }
  84% {
    width: 8px;
    left: 10px;
    top: 20px; }
  100% {
    width: 12px;
    left: 3px;
    top: 18px; } }

@keyframes animate-toast-success-tip {
  0% {
    width: 0;
    left: 1px;
    top: 9px; }
  54% {
    width: 0;
    left: 1px;
    top: 9px; }
  70% {
    width: 24px;
    left: -4px;
    top: 17px; }
  84% {
    width: 8px;
    left: 10px;
    top: 20px; }
  100% {
    width: 12px;
    left: 3px;
    top: 18px; } }

@-webkit-keyframes animate-toast-success-long {
  0% {
    width: 0;
    right: 22px;
    top: 26px; }
  65% {
    width: 0;
    right: 22px;
    top: 26px; }
  84% {
    width: 26px;
    right: 0;
    top: 15px; }
  100% {
    width: 22px;
    right: 3px;
    top: 15px; } }

@keyframes animate-toast-success-long {
  0% {
    width: 0;
    right: 22px;
    top: 26px; }
  65% {
    width: 0;
    right: 22px;
    top: 26px; }
  84% {
    width: 26px;
    right: 0;
    top: 15px; }
  100% {
    width: 22px;
    right: 3px;
    top: 15px; } }

@-webkit-keyframes rotatePlaceholder {
  0% {
    -webkit-transform: rotate(-45deg);
            transform: rotate(-45deg); }
  5% {
    -webkit-transform: rotate(-45deg);
            transform: rotate(-45deg); }
  12% {
    -webkit-transform: rotate(-405deg);
            transform: rotate(-405deg); }
  100% {
    -webkit-transform: rotate(-405deg);
            transform: rotate(-405deg); } }

@keyframes rotatePlaceholder {
  0% {
    -webkit-transform: rotate(-45deg);
            transform: rotate(-45deg); }
  5% {
    -webkit-transform: rotate(-45deg);
            transform: rotate(-45deg); }
  12% {
    -webkit-transform: rotate(-405deg);
            transform: rotate(-405deg); }
  100% {
    -webkit-transform: rotate(-405deg);
            transform: rotate(-405deg); } }

.swal2-animate-success-line-tip {
  -webkit-animation: animate-success-tip .75s;
          animation: animate-success-tip .75s; }

.swal2-animate-success-line-long {
  -webkit-animation: animate-success-long .75s;
          animation: animate-success-long .75s; }

.swal2-success.swal2-animate-success-icon .swal2-success-circular-line-right {
  -webkit-animation: rotatePlaceholder 4.25s ease-in;
          animation: rotatePlaceholder 4.25s ease-in; }

@-webkit-keyframes animate-error-icon {
  0% {
    -webkit-transform: rotateX(100deg);
            transform: rotateX(100deg);
    opacity: 0; }
  100% {
    -webkit-transform: rotateX(0deg);
            transform: rotateX(0deg);
    opacity: 1; } }

@keyframes animate-error-icon {
  0% {
    -webkit-transform: rotateX(100deg);
            transform: rotateX(100deg);
    opacity: 0; }
  100% {
    -webkit-transform: rotateX(0deg);
            transform: rotateX(0deg);
    opacity: 1; } }

.swal2-animate-error-icon {
  -webkit-animation: animate-error-icon .5s;
          animation: animate-error-icon .5s; }

@-webkit-keyframes animate-x-mark {
  0% {
    -webkit-transform: scale(0.4);
            transform: scale(0.4);
    margin-top: 26px;
    opacity: 0; }
  50% {
    -webkit-transform: scale(0.4);
            transform: scale(0.4);
    margin-top: 26px;
    opacity: 0; }
  80% {
    -webkit-transform: scale(1.15);
            transform: scale(1.15);
    margin-top: -6px; }
  100% {
    -webkit-transform: scale(1);
            transform: scale(1);
    margin-top: 0;
    opacity: 1; } }

@keyframes animate-x-mark {
  0% {
    -webkit-transform: scale(0.4);
            transform: scale(0.4);
    margin-top: 26px;
    opacity: 0; }
  50% {
    -webkit-transform: scale(0.4);
            transform: scale(0.4);
    margin-top: 26px;
    opacity: 0; }
  80% {
    -webkit-transform: scale(1.15);
            transform: scale(1.15);
    margin-top: -6px; }
  100% {
    -webkit-transform: scale(1);
            transform: scale(1);
    margin-top: 0;
    opacity: 1; } }

.swal2-animate-x-mark {
  -webkit-animation: animate-x-mark .5s;
          animation: animate-x-mark .5s; }

@-webkit-keyframes rotate-loading {
  0% {
    -webkit-transform: rotate(0deg);
            transform: rotate(0deg); }
  100% {
    -webkit-transform: rotate(360deg);
            transform: rotate(360deg); } }

@keyframes rotate-loading {
  0% {
    -webkit-transform: rotate(0deg);
            transform: rotate(0deg); }
  100% {
    -webkit-transform: rotate(360deg);
            transform: rotate(360deg); } }
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}iframe#_hjRemoteVarsFrame {display: none !important; width: 1px !important; height: 1px !important; opacity: 0 !important; pointer-events: none !important;}


&lt;iframe src=&quot;https://www.googletagmanager.com/ns.html?id=GTM-WKTB94R&quot;
height=&quot;0&quot; width=&quot;0&quot; style=&quot;display:none;visibility:hidden&quot;>&lt;/iframe>


    
        
            
                
                
                Multichannel Chat
            
            
                Already have an account? Sign in →
            
        
        Create Your Account Company Name   Industry Select your industry AutomotiveBanking &amp; FinanceConsumer GoodsPharmaceuticalEnergyMiningGovernmentHealthcareInsuranceManufacturingMedia &amp; EntertainmentRetailPropertyTransportationTelecommunicationsEducationOther The Industry field is required. Work Email   Phone Number   Password   Repeat Password   Register → Auto free trial for 14 days!* Up to 1,000 Monthly Active Users Up to 4 channels, excluding WhatsApp Up to 5 Agents Chat Templates Basic standard support *Have the benefits of our Essentials plan ($50/month) for free in 14 days after your create your new account
    
    
    
    
    
    
        Vue.use(VeeValidate);
        new Vue({
            el: '.qismo-content',
            data: {
                company_name: '',
                work_email: '',
                password: '',
                phone_number: '',
                confirmation: '',
                isProcessing: false,
                baseURL: '//multichannel.qiscus.com/api/v1',
                selectedIndustry: null,
                optionIndustry: [
                    {
                        'text': 'Automotive',
                        'value': 'Automotive',
                    },
                    {
                        'text': 'Banking &amp; Finance',
                        'value': 'Banking &amp; Finance',
                    },
                    {
                        'text': 'Consumer Goods',
                        'value': 'Consumer Goods',
                    },
                    {
                        'text': 'Pharmaceutical',
                        'value': 'Pharmaceutical',
                    },
                    {
                        'text': 'Energy',
                        'value': 'Energy',
                    },
                    {
                        'text': 'Mining',
                        'value': 'Mining',
                    },
                    {
                        'text': 'Government',
                        'value': 'Government',
                    },
                    {
                        'text': 'Healthcare',
                        'value': 'Healthcare',
                    },
                    {
                        'text': 'Insurance',
                        'value': 'Insurance',
                    },
                    {
                        'text': 'Manufacturing',
                        'value': 'Manufacturing',
                    },
                    {
                        'text': 'Media &amp; Entertainment',
                        'value': 'Media &amp; Entertainment',
                    },
                    {
                        'text': 'Retail',
                        'value': 'Retail',
                    },
                    {
                        'text': 'Property',
                        'value': 'Property',
                    },
                    {
                        'text': 'Transportation',
                        'value': 'Transportation',
                    },
                    {
                        'text': 'Telecommunications',
                        'value': 'Telecommunications',
                    },
                    {
                        'text': 'Education',
                        'value': 'Education',
                    },
                    {
                        'text': 'Other',
                        'value': 'Other',
                    },
                ],
                showErrorIndustry: false,
            },
            mounted() {
                this.$validator.pause();
            },
            methods: {
                register: function() {
                    this.$validator.resume();
                    this.$validator.validateAll().then(val => {
                        if (!this.selectedIndustry) return this.showErrorIndustry = true;
                        if (!val) return;
                        this.isProcessing = true;
                        swal({ text: 'Please wait...', allowOutsideClick: false });
                        swal.showLoading();
                        const data = {
                            name: this.company_name,
                            email: this.work_email,
                            password: this.password,
                            phone: this.phone_number,
                            industry: this.selectedIndustry,
                        };
                        this.$http.post(`${this.baseURL}/sign_up`, data).then(res => {
                            console.log(res);
                            this.phone_number = '';
                            this.company_name = '';
                            this.work_email = '';
                            this.password = '';
                            this.confirmation = '';
                            this.isProcessing = false;
                            this.selectedIndustry = null;
                            swal({
                                title: 'Success',
                                text: 'Registration success, we\'ve sent you a verification email to verify your account',
                                type: 'success',
                                allowOutsideClick: false
                            })
                            setTimeout(function() { 
                                window.location.href = 'https://multichannel.qiscus.com';
                            }, 3000);

                        }, err => {
                            this.$validator.pause();
                            this.isProcessing = false;
                            swal({
                                title: 'Failed',
                                text: err.body.errors,
                                type: 'error',
                                allowOutsideClick: false
                            })
                        });
                    })
                }
            },
        });
    

window.NREUM||(NREUM={});NREUM.info={&quot;beacon&quot;:&quot;bam.nr-data.net&quot;,&quot;licenseKey&quot;:&quot;b9f2942ac8&quot;,&quot;applicationID&quot;:&quot;246424479&quot;,&quot;transactionName&quot;:&quot;b1BaMBMFWURQARdaDlYaeQcVDVhZHg8WXxVRVlAFDwpSW24QBlQIS0FdFg==&quot;,&quot;queueTime&quot;:11,&quot;applicationTime&quot;:17,&quot;atts&quot;:&quot;QxdZRlsfSko=&quot;,&quot;errorBeacon&quot;:&quot;bam.nr-data.net&quot;,&quot;agent&quot;:&quot;&quot;}
(function(c){function d(a){if(!(this instanceof d))return new d(a);a=a||{};var b=a.context||&quot;body&quot;;&quot;string&quot;===typeof b&amp;&amp;(b=h.querySelector(b));if(!b)throw Error(&quot;Unable to find context &quot;+b);this._context=b;this.minHeight=a.minHeight||0;this._marks={};this._tracked={};this._config={percentages:{each:{},every:{}},pixels:{each:{},every:{}},elements:{each:{},every:{}}};a=n(this._checkDepth.bind(this),500);b=this._update.bind(this);var g=n(b,500);c.addEventListener(&quot;scroll&quot;,a,!0);c.addEventListener(&quot;resize&quot;,
g);this._artifacts={timer:q(b),resize:g,scroll:a}}function r(a){return a.handlers.map(function(b){return b.bind(this,{data:{depth:a.depth,label:a.label}})})}function p(a){var b=Math.floor(a.numerator/a.n),g;for(g=1;g&lt;=b;g++)a.callback(g*a.n)}function q(a){var b=m();return setInterval(function(){m()!==b&amp;&amp;(a(),b=m())},500)}function m(){var a=h.body,b=h.documentElement;return Math.max(a.scrollHeight,a.offsetHeight,b.clientHeight,b.scrollHeight,b.offsetHeight)}function t(a){a=a.getBoundingClientRect().top-
(a.scrollHeight-a.clientHeight)/2;var b=void 0!==c.pageYOffset?c.pageYOffset:(h.documentElement||h.body.parentNode||h.body).scrollTop;return a+b}function u(){}function n(a,b){var g,e,d,l=null,c=0,f=function(){c=new Date;l=null;d=a.apply(g,e)};return function(){var k=new Date;c||(c=k);var h=b-(k-c);g=this;e=arguments;0>=h?(clearTimeout(l),l=null,c=k,d=a.apply(g,e)):l||(l=setTimeout(f,h));return d}}function v(){var a={},b;for(b in d)a[b]=u;c.ScrollTracker=a}if(c.navigator.userAgent.match(/MSIE [678]/gi))return v();
var h=c.document;d.prototype.destroy=function(){clearInterval(this._artifacts._timer);c.removeEventListener(&quot;resize&quot;,this._artifacts.resize);c.removeEventListener(&quot;scroll&quot;,this._artifacts.scroll,!0)};d.prototype.on=function(a,b){var g=this._config;[&quot;percentages&quot;,&quot;pixels&quot;,&quot;elements&quot;].forEach(function(e){a[e]&amp;&amp;[&quot;each&quot;,&quot;every&quot;].forEach(function(c){a[e][c]&amp;&amp;a[e][c].forEach(function(a){g[e][c][a]=g[e][c][a]||[];g[e][c][a].push(b)})})});this._update()};d.prototype._update=function(){this._calculateMarks();
this._checkDepth()};d.prototype._calculateMarks=function(){function a(a,b){return function(b,c){var g=b.getBoundingClientRect().top-h._context.getBoundingClientRect().top;d({label:a+&quot;[&quot;+c+&quot;]&quot;,depth:g,handlers:e.elements.every[a]})}}function b(a){return function(a){var b=Math.floor(a*c/100);d({label:String(a)+&quot;%&quot;,depth:b,handlers:e.percentages.every[f]})}}function g(a){return function(b){d({label:String(b)+&quot;px&quot;,depth:b,handlers:a})}}delete this._marks;this._fromTop=t(this._context);this._marks={};
var e=this._config,c=this._contextHeight(),d=this._addMark.bind(this),h=this,f;if(!(c&lt;this.minHeight)){for(f in e.percentages.every)p({n:Number(f),numerator:100,callback:b(e.percentages.every[f])});for(f in e.pixels.every)p({n:Number(f),numerator:c,callback:g(e.pixels.every[f])});for(f in e.percentages.each){var k=Math.floor(c*Number(f)/100);d({label:f+&quot;%&quot;,depth:k,handlers:e.percentages.each[f]})}for(f in e.pixels.each)k=Number(f),d({label:f+&quot;px&quot;,depth:k,handlers:e.pixels.each[f]});for(f in e.elements.every)k=
[].slice.call(this._context.querySelectorAll(f)),k.length&amp;&amp;k.forEach(a(f,e.elements.every[f]));for(f in e.elements.each)if(k=this._context.querySelector(f))k=k.getBoundingClientRect().top-h._context.getBoundingClientRect().top,d({label:f,depth:k,handlers:e.elements.each[f]})}};d.prototype._checkDepth=function(){var a=this._marks,b=this._currentDepth(),c;for(c in a)b>=c&amp;&amp;!this._tracked[c]&amp;&amp;(a[c].forEach(function(a){a()}),this._tracked[c]=!0)};d.prototype.reset=function(){this._tracked={};this._marks=
{};this._update()};d.prototype._contextHeight=function(){return this._context!==h.body?this._context.scrollHeight-5:this._context.clientHeight-5};d.prototype._currentDepth=function(){var a=this._context;var b=a.offsetHeight;var d=&quot;CSS1Compat&quot;===h.compatMode?h.documentElement:h.body;d=d.clientHeight;a=a.getBoundingClientRect();b=Math.max(0,0&lt;a.top?Math.min(b,d-a.top):a.bottom&lt;d?a.bottom:d);this._context.scrollTop?a=this._context.scrollTop+b:(this._context.scrollTop=1,this._context.scrollTop?(this._context.scrollTop=
0,a=this._context.scrollTop+b):a=(c.pageYOffset||h.documentElement.scrollTop||h.body.scrollTop||0)-this._fromTop);return b?a+b:a>=this._fromTop?a:-1};d.prototype._addMark=function(a){var b=a.depth;this._marks[b]=(this._marks[b]||[]).concat(r(a))};c.ScrollTracker=d})(this);
(function(c){function d(){var d=c.ScrollTracker();d.on({percentages:{each:[10,90],every:[25]}},function(c){dataLayer.push({event:&quot;scrollTracking&quot;,attributes:{pixels:c.data.depth,distance:c.data.label,label:google_tag_manager[&quot;GTM-WKTB94R&quot;].macro(3)}})});delete c.ScrollTracker}&quot;loading&quot;!==document.readyState?d():document.addEventListener(&quot;DOMContentLoaded&quot;,d)})(window);
!function(b,e,f,g,a,c,d){b.fbq||(a=b.fbq=function(){a.callMethod?a.callMethod.apply(a,arguments):a.queue.push(arguments)},b._fbq||(b._fbq=a),a.push=a,a.loaded=!0,a.version=&quot;2.0&quot;,a.queue=[],c=e.createElement(f),c.async=!0,c.src=g,d=e.getElementsByTagName(f)[0],d.parentNode.insertBefore(c,d))}(window,document,&quot;script&quot;,&quot;https://connect.facebook.net/en_US/fbevents.js&quot;);fbq(&quot;init&quot;,&quot;458890707844449&quot;);fbq(&quot;set&quot;,&quot;agent&quot;,&quot;tmgoogletagmanager&quot;,&quot;458890707844449&quot;);fbq(&quot;track&quot;,&quot;PageView&quot;);
&lt;img height=&quot;1&quot; width=&quot;1&quot; style=&quot;display:none&quot; src=&quot;https://www.facebook.com/tr?id=458890707844449&amp;amp;ev=PageView&amp;amp;noscript=1&quot;>
/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
