(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[459],{6429:function(n,e,t){(window.__NEXT_P=window.__NEXT_P||[]).push(["/login",function(){return t(768)}])},768:function(n,e,t){"use strict";t.r(e);var o=t(5893),r=t(8976),a=t(7294),s=t(1163),i=t(270),u=t.n(i);e.default=()=>{let n=(0,s.useRouter)(),[e,t]=(0,a.useState)(""),[i,c]=(0,a.useState)(""),[l,d]=(0,a.useState)(!1),{login:m}=(0,r.a)(),p=async t=>{t.preventDefault(),d(!0);let o=await m(e,i);d(!1),o?n.push("/"):alert("Login failed.")};return(0,o.jsx)("div",{children:(0,o.jsx)("form",{id:"".concat(u().loginForm),onSubmit:p,method:"post",children:(0,o.jsxs)("div",{className:"".concat(u().container),children:[(0,o.jsx)("label",{htmlFor:"name",children:(0,o.jsx)("b",{children:"Username"})}),(0,o.jsx)("input",{id:"name",type:"text",className:"".concat(u().formInput),placeholder:"Enter Username",name:"name",onChange:n=>{t(n.target.value)},required:!0}),(0,o.jsx)("label",{htmlFor:"password",children:(0,o.jsx)("b",{children:"Password"})}),(0,o.jsx)("input",{id:"password",type:"password",className:"".concat(u().formInput),placeholder:"Enter Password",name:"password",onChange:n=>{c(n.target.value)},disabled:l,required:!0}),(0,o.jsx)("button",{className:"".concat(u().formButton),type:"submit",children:"Login"})]})})})}},270:function(n){n.exports={loginForm:"Login_loginForm__Es0pH",formInput:"Login_formInput__0DoZd",formButton:"Login_formButton__nATKd",container:"Login_container__DPp2Z"}},1163:function(n,e,t){n.exports=t(8355)}},function(n){n.O(0,[774,888,179],function(){return n(n.s=6429)}),_N_E=n.O()}]);