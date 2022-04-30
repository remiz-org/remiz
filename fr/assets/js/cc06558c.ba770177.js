"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[18],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return f}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var s=r.createContext({}),u=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},p=function(e){var t=u(e.components);return r.createElement(s.Provider,{value:t},e.children)},l={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,s=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),d=u(n),f=o,m=d["".concat(s,".").concat(f)]||d[f]||l[f]||i;return n?r.createElement(m,a(a({ref:t},p),{},{components:n})):r.createElement(m,a({ref:t},p))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,a=new Array(i);a[0]=d;var c={};for(var s in t)hasOwnProperty.call(t,s)&&(c[s]=t[s]);c.originalType=e,c.mdxType="string"==typeof e?e:o,a[1]=c;for(var u=2;u<i;u++)a[u]=n[u];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},4694:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return c},contentTitle:function(){return s},metadata:function(){return u},toc:function(){return p},default:function(){return d}});var r=n(7462),o=n(3366),i=(n(7294),n(3905)),a=["components"],c={sidebar_position:1},s="Introduction",u={unversionedId:"intro",id:"intro",title:"Introduction",description:"D\xe9couvrons ensemble ce que vous pouvez faire avec Remiz.",source:"@site/i18n/fr/docusaurus-plugin-content-docs/current/intro.mdx",sourceDirName:".",slug:"/intro",permalink:"/remiz/fr/docs/intro",editUrl:"https://github.com/remiz-org/remiz/tree/main/docs/docs/intro.mdx",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"tutorialSidebar",next:{title:"Installation",permalink:"/remiz/fr/docs/tutorial/installation"}},p=[{value:"A propos du format .pack",id:"a-propos-du-format-pack",children:[],level:3}],l={toc:p};function d(e){var t=e.components,n=(0,o.Z)(e,a);return(0,i.kt)("wrapper",(0,r.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"introduction"},"Introduction"),(0,i.kt)("p",null,"D\xe9couvrons ensemble ce que vous pouvez faire avec ",(0,i.kt)("strong",{parentName:"p"},"Remiz"),"."),(0,i.kt)("p",null,'Remiz est un outil simple (mais extensible) en ligne de commande pour cr\xe9er\net d\xe9ployer des copies reproductibles de vos projets.\nToutes les donn\xe9es sont s\xe9rialis\xe9es dans un seul fichier binaire compress\xe9,\ndont l\'extension est ".pack" (personnalisable).'),(0,i.kt)("p",null,"Cet outil est facilement int\xe9grable dans une pipeline de CI/CD (Gitlab,\nJenkins, ...) pour d\xe9crire le processus de construction et de d\xe9ploiement via\ndes fichiers de configuration."),(0,i.kt)("p",null,"Remiz est open source (sous MIT), multi-plateforme (Windows, Mac, Linux),\nrapide et pleinement personnalisable \xe0 l'aide de fichiers de configuration TOML."),(0,i.kt)("h3",{id:"a-propos-du-format-pack"},"A propos du format .pack"),(0,i.kt)("p",null,"Remiz s'appuie sur le format Multi Layer Archive (plus d'informations : ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/ANSSI-FR/MLA"},"MLA"),")\nIl s'agit d'un format bas\xe9 sur la compression Brotli (d\xe9velopp\xe9 par l'\xe9quipe de chez Dropbox) et supporte\nle chiffrement.\nMLA est d\xe9velopp\xe9 par l'ANSSI \ud83c\uddeb\ud83c\uddf7 (Agence Nationalise de la Securit\xe9 des Syst\xe8mes d'Information)."))}d.isMDXComponent=!0}}]);