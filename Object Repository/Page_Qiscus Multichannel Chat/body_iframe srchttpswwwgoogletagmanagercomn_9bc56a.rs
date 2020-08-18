<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_iframe srchttpswwwgoogletagmanagercomn_9bc56a</name>
   <tag></tag>
   <elementGuidId>3a1dcbfa-97da-47b3-8231-31d2cb0a8624</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

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
    

window.NREUM||(NREUM={});NREUM.info={&quot;beacon&quot;:&quot;bam.nr-data.net&quot;,&quot;licenseKey&quot;:&quot;b9f2942ac8&quot;,&quot;applicationID&quot;:&quot;246424479&quot;,&quot;transactionName&quot;:&quot;b1BaMBMFWURQARdaDlYaeQcVDVhZHg8WXxVRVlAFDwpSW24QBlQIS0FdFg==&quot;,&quot;queueTime&quot;:5,&quot;applicationTime&quot;:15,&quot;atts&quot;:&quot;QxdZRlsfSko=&quot;,&quot;errorBeacon&quot;:&quot;bam.nr-data.net&quot;,&quot;agent&quot;:&quot;&quot;}
/html[1]/body[1](function(c){function d(a){if(!(this instanceof d))return new d(a);a=a||{};var b=a.context||&quot;body&quot;;&quot;string&quot;===typeof b&amp;&amp;(b=h.querySelector(b));if(!b)throw Error(&quot;Unable to find context &quot;+b);this._context=b;this.minHeight=a.minHeight||0;this._marks={};this._tracked={};this._config={percentages:{each:{},every:{}},pixels:{each:{},every:{}},elements:{each:{},every:{}}};a=n(this._checkDepth.bind(this),500);b=this._update.bind(this);var g=n(b,500);c.addEventListener(&quot;scroll&quot;,a,!0);c.addEventListener(&quot;resize&quot;,
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
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
