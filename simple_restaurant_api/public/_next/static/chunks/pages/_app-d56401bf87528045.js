(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[888],{1118:function(e,t,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/_app",function(){return n(7648)}])},8976:function(e,t,n){"use strict";n.d(t,{H:function(){return a},a:function(){return i}});var r=n(5893),o=n(7294);let u=(0,o.createContext)(),a=e=>{let{children:t}=e,[n,a]=(0,o.useState)(null),[i,l]=(0,o.useState)(!0),c=async(e,t)=>{try{let n=await fetch("/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({name:e,password:t})});if(!n.ok)return console.error("Login failed"),!1;let r=await n.json();return r.sub=r.name,console.log(r),window.localStorage.setItem("login_token",r.token),a(r),!0}catch(e){return console.error("Login error:",e),!1}},s=async()=>{try{let e=window.localStorage.getItem("login_token");if(""===e||"undefined"===e)return console.log(e),!1;let t=await fetch("/api/auth/current_user",{method:"GET",headers:{Authorization:"Bearer ".concat(e)}});if(!t.ok)return!1;{let n=await t.json();return n.token=e,n}}catch(e){return console.error("Failed to fetch current user",e),!1}};(0,o.useEffect)(()=>{let e=async()=>{try{let e=await s();e?a(e):(console.log("Failed to load user"),a(null))}catch(e){console.error("Failed to fetch current user",e),a(null)}finally{l(!1)}};e()},[a,l]);let f=async()=>{try{return localStorage.removeItem("login_token"),a(null),!0}catch(e){return!1}};return(0,r.jsx)(u.Provider,{value:{user:n,setUser:a,loading:i,login:c,logout:f},children:t})},i=()=>{let e=(0,o.useContext)(u);if(void 0===e)throw Error("useAuth must be used within an AuthProvider");return e}},7648:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return u}});var r=n(5893);n(5303);var o=n(8976);function u(e){let{Component:t,pageProps:n}=e;return(0,r.jsx)(o.H,{children:(0,r.jsx)(t,{...n})})}},5303:function(){}},function(e){var t=function(t){return e(e.s=t)};e.O(0,[774,179],function(){return t(1118),t(8355)}),_N_E=e.O()}]);