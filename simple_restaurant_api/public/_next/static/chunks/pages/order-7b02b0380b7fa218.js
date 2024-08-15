(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[941],{3376:function(e,t,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/order",function(){return n(9923)}])},9923:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return g}});var r=n(5893),o=n(9511),a=n.n(o),l=n(9008),c=n.n(l),i=n(6154),d=n(1338),s=n.n(d),u=n(7294),h=n(1163),_=n(8976),p=e=>{let{targetTimeString:t}=e,[n,o]=(0,u.useState)(null);return((0,u.useEffect)(()=>{let e=setInterval(()=>{let n=new Date,r=new Date(t),a=r-n;a<=0?(clearInterval(e),o({minutes:0,seconds:0,finished:!0})):o({minutes:Math.floor(a/1e3/60)%60,seconds:Math.floor(a/1e3)%60,finished:!1})},1e3);return()=>clearInterval(e)},[t]),null===n)?(0,r.jsx)("div",{children:"Loading..."}):n.finished?(0,r.jsx)("div",{children:"Finished"}):(0,r.jsxs)("div",{children:[n.minutes,"min ",n.seconds,"sec"]})};let f=e=>"".concat(Math.floor(e/60)," minutes"),x=e=>{let t=new Date(j(e)),n=t.toLocaleString();return n},j=e=>e.endsWith("Z")?e:e+"Z";var m=e=>{let{user:t,onClose:n,orderId:o}=e,[a,l]=(0,u.useState)(!0),[c,d]=(0,u.useState)(null);return(0,u.useEffect)(()=>{let e=async e=>{console.log("Order ID:",e),await i.Z.get("/api/order/".concat(e),{headers:{Authorization:"Bearer ".concat(t.token),"Content-Type":"application/json"}}).then(e=>{console.log(e),d(e.data),l(!1)}).catch(e=>{console.error("Error fetching data: ",e),alert("Error fetching data: ",e),l(!1)})};e(o)},[o]),(0,r.jsx)("div",{style:{position:"fixed",top:0,left:0,right:0,bottom:0,backgroundColor:"rgba(0,0,0,0.5)",display:"flex",justifyContent:"center",alignItems:"center"},children:(0,r.jsxs)("div",{style:{backgroundColor:"white",padding:"20px",borderRadius:"8px"},children:[a?(0,r.jsx)("p",{children:"Loading..."}):(0,r.jsxs)("div",{children:[(0,r.jsx)("div",{children:c.menu_name}),(0,r.jsxs)("div",{children:["Price: ",c.price," yen"]}),(0,r.jsxs)("div",{children:["expected to take"," ",f(c.cook_time_seconds)]}),(0,r.jsxs)("div",{children:["Ordered at: ",x(c.ordered_time)]}),(0,r.jsxs)("div",{children:["Checked by: ",c.check_staff_name]})]}),(0,r.jsx)("br",{}),(0,r.jsx)("button",{onClick:n,children:"Close Modal"})]})})};function g(){let e=(0,h.useRouter)(),{user:t,loading:n}=(0,_.a)(),[o,l]=(0,u.useState)([]),[d,f]=(0,u.useState)(null),[x,j]=(0,u.useState)(null),[g,b]=(0,u.useState)(!1),[k,y]=(0,u.useState)(!1);(0,u.useEffect)(()=>{let{tableId:r}=e.query;n||t||e.push("/login"),!n&&t&&r&&(j(r),i.Z.get("/api/table/".concat(r,"/order"),{headers:{Authorization:"Bearer ".concat(t.token)}}).then(e=>{console.log(e.data),l(e.data)}).catch(e=>{alert("Error fetching data: ",e)}))},[t,n,e]);let v=(e,t)=>{f(e,t),y(!0)},C=e=>{b(!0),console.log("Order ID:",e),i.Z.delete("/api/order",{headers:{Authorization:"Bearer ".concat(t.token),"Content-Type":"application/json"},data:{order_id:parseInt(e)}}).then(t=>{console.log(t.data),l(t=>t.filter(t=>t.order_id!==e)),b(!1),alert("Successfully canceled")}).catch(e=>{b(!1),alert("Error fetching data: ",e)})},O=(e,n)=>{b(!0),console.log("Order ID:",e),i.Z.delete("/api/order/complete",{headers:{Authorization:"Bearer ".concat(t.token),"Content-Type":"application/json"},data:{order_id:parseInt(e)}}).then(t=>{console.log(t.data),l(t=>t.filter(t=>t.order_id!==e)),b(!1),alert('Successfully served "'.concat(n,'"'))}).catch(e=>{b(!1),alert("Error fetching data: ",e)})},w=e=>{b(!0),console.log("Table ID:",e),i.Z.delete("/api/table/order",{headers:{Authorization:"Bearer ".concat(t.token),"Content-Type":"application/json"},data:{restaurant_table_id:parseInt(e)}}).then(e=>{console.log(e.data),l([]),b(!1),alert("Successfully canceled all orders")}).catch(e=>{b(!1),alert("Error fetching data: ",e)})},I=t=>{console.log("Table ID:",t),e.push("/menu?tableId=".concat(t))},S=e=>{let t=new Date(E(e)),n=t.toLocaleString();return n},E=e=>e.endsWith("Z")?e:e+"Z";return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsxs)(c(),{children:[(0,r.jsx)("title",{children:"Restaurant App"}),(0,r.jsx)("meta",{name:"description",content:"Restaurant App"}),(0,r.jsx)("meta",{name:"viewport",content:"width=device-width, initial-scale=1"}),(0,r.jsx)("link",{rel:"icon",href:"/favicon.ico"})]}),(0,r.jsxs)("main",{className:"".concat(s().main," ").concat(a().className),children:[(0,r.jsxs)("h2",{children:["Table ",x]}),k&&(0,r.jsx)(m,{user:t,onClose:()=>{y(!1)},orderId:d}),(0,r.jsx)("ul",{children:0===o.length?(0,r.jsx)("div",{children:"No order found"}):o.map((e,t)=>(0,r.jsx)("li",{className:"".concat(s().order),children:(0,r.jsxs)("ul",{children:[(0,r.jsx)("li",{children:e.menu_name}),(0,r.jsxs)("li",{children:["Expected to finish cooking at:"," ",S(e.expected_cook_finish_time)]}),(0,r.jsx)("li",{children:(0,r.jsx)(p,{targetTimeString:S(e.expected_cook_finish_time)})}),(0,r.jsxs)("li",{children:[(0,r.jsx)("button",{className:"".concat(s().deleteOrderButton),type:"button",onClick:()=>C(e.order_id),disabled:g,children:"Cancel order"}),(0,r.jsx)("button",{className:"".concat(s().deleteOrderButton),type:"button",disabled:g,onClick:()=>O(e.order_id,e.menu_name),children:"Serve"}),(0,r.jsx)("button",{className:"".concat(s().deleteOrderButton),type:"button",disabled:g,onClick:()=>v(e.order_id),children:"Detail"})]})]})},t))}),(0,r.jsxs)("div",{children:[(0,r.jsx)("button",{type:"button",onClick:()=>I(x),children:"Menu"}),(0,r.jsx)("button",{type:"button",className:"".concat(s().deleteOrderButton),onClick:()=>w(x),children:"Delete all orders"})]})]})]})}},9511:function(e){e.exports={style:{fontFamily:"'__Inter_e66fe9', '__Inter_Fallback_e66fe9'",fontStyle:"normal"},className:"__className_e66fe9"}},1338:function(e){e.exports={addOrderButton:"Order_addOrderButton__zKuom",deleteOrderButton:"Order_deleteOrderButton__bKyQ7",order:"Order_order__wF0KT"}}},function(e){e.O(0,[45,774,888,179],function(){return e(e.s=3376)}),_N_E=e.O()}]);