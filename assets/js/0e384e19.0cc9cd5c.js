"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[671],{3905:function(e,t,n){n.d(t,{Zo:function(){return s},kt:function(){return m}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),u=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},s=function(e){var t=u(e.components);return r.createElement(l.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,l=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),d=u(n),m=o,f=d["".concat(l,".").concat(m)]||d[m]||p[m]||i;return n?r.createElement(f,a(a({ref:t},s),{},{components:n})):r.createElement(f,a({ref:t},s))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,a=new Array(i);a[0]=d;var c={};for(var l in t)hasOwnProperty.call(t,l)&&(c[l]=t[l]);c.originalType=e,c.mdxType="string"==typeof e?e:o,a[1]=c;for(var u=2;u<i;u++)a[u]=n[u];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},9881:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return c},contentTitle:function(){return l},metadata:function(){return u},toc:function(){return s},default:function(){return d}});var r=n(7462),o=n(3366),i=(n(7294),n(3905)),a=["components"],c={sidebar_position:1},l="About Remiz",u={unversionedId:"intro",id:"intro",title:"About Remiz",description:"Let's discover in this page what you can do with Remiz.",source:"@site/docs/intro.md",sourceDirName:".",slug:"/intro",permalink:"/remiz/docs/intro",editUrl:"https://github.com/remiz-org/remiz/tree/main/docs/docs/intro.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"tutorialSidebar",next:{title:"Installation",permalink:"/remiz/docs/tutorial/installation"}},s=[{value:"Introduction",id:"introduction",children:[{value:"The .pack format",id:"the-pack-format",children:[],level:3}],level:2}],p={toc:s};function d(e){var t=e.components,n=(0,o.Z)(e,a);return(0,i.kt)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"about-remiz"},"About Remiz"),(0,i.kt)("p",null,"Let's discover in this page what you can do with ",(0,i.kt)("strong",{parentName:"p"},"Remiz"),"."),(0,i.kt)("h2",{id:"introduction"},"Introduction"),(0,i.kt)("p",null,'Remiz is a simple (but extensible) command line tool to create and deploy reproducible copy of your projects. All the data is serialized into a single compressed binary file ending with ".pack" (customizable).'),(0,i.kt)("p",null,"This tool could be used inside a CI/CD pipeline (Gitlab, Jenkins, ...) to describe the packaging and deployment process or locally on a non versionned project."),(0,i.kt)("p",null,"Remiz is open source (MIT licensed), cross platform, fast and fully customizable with TOML configurations file."),(0,i.kt)("h3",{id:"the-pack-format"},"The .pack format"),(0,i.kt)("p",null,"Remiz stands on Multi Layer Archive format (more info : ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/ANSSI-FR/MLA"},"MLA"),").\nIt's based on Brotli compression (developped by the Dropbox team) and supports encryption.\nMLA is developped by the ANSSI (French National Agency for the Security of Information Systems)."))}d.isMDXComponent=!0}}]);