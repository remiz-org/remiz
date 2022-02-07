"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[63],{3905:function(e,t,n){n.d(t,{Zo:function(){return u},kt:function(){return f}});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var c=r.createContext({}),s=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},u=function(e){var t=s(e.components);return r.createElement(c.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,c=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),m=s(n),f=a,d=m["".concat(c,".").concat(f)]||m[f]||p[f]||o;return n?r.createElement(d,i(i({ref:t},u),{},{components:n})):r.createElement(d,i({ref:t},u))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=m;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l.mdxType="string"==typeof e?e:a,i[1]=l;for(var s=2;s<o;s++)i[s]=n[s];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},6212:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return c},metadata:function(){return s},toc:function(){return u},default:function(){return m}});var r=n(7462),a=n(3366),o=(n(7294),n(3905)),i=["components"],l={sidebar_position:1},c="Installation",s={unversionedId:"tutorial/installation",id:"tutorial/installation",title:"Installation",description:"Remiz is available for Windows, Mac and Linux. You can download the latest",source:"@site/i18n/fr/docusaurus-plugin-content-docs/current/tutorial/installation.md",sourceDirName:"tutorial",slug:"/tutorial/installation",permalink:"/remiz/fr/docs/tutorial/installation",editUrl:"https://github.com/remiz-org/remiz/tree/main/docs/docs/tutorial/installation.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"tutorialSidebar",previous:{title:"Introduction",permalink:"/remiz/fr/docs/intro"},next:{title:"Set up global configuration",permalink:"/remiz/fr/docs/tutorial/global-config"}},u=[{value:"Extract the binary from the archive",id:"extract-the-binary-from-the-archive",children:[],level:2},{value:"Choose a location for the binary",id:"choose-a-location-for-the-binary",children:[],level:2},{value:"Test your installation",id:"test-your-installation",children:[],level:2}],p={toc:u};function m(e){var t=e.components,n=(0,a.Z)(e,i);return(0,o.kt)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"installation"},"Installation"),(0,o.kt)("p",null,(0,o.kt)("strong",{parentName:"p"},"Remiz")," is available for Windows, Mac and Linux. You can download the latest\nversion of the binary from the ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/remiz-org/remiz/releases/latest"},"Github Release page")),(0,o.kt)("h2",{id:"extract-the-binary-from-the-archive"},"Extract the binary from the archive"),(0,o.kt)("p",null,"On the release page, you'll find the link to an archive. For example, for Linux,\nyou may want to choose the .tar.gz download option and may want to execute\nthe following command :"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre"},"wget https://github.com/remiz-org/remiz/releases/download/v0.0.17/remiz_v0.0.17_x86_64-unknown-linux-musl.tar.gz\ntar -xf remiz_v0.0.17_x86_64-unknown-linux-musl.tar.gz\n")),(0,o.kt)("h2",{id:"choose-a-location-for-the-binary"},"Choose a location for the binary"),(0,o.kt)("p",null,"Once the binary extracted, you may want to choose a final location for installation.\nOn Linux, many users will place the binary inside the ",(0,o.kt)("inlineCode",{parentName:"p"},"/usr/local/bin")," since\nit's contained in the ",(0,o.kt)("inlineCode",{parentName:"p"},"$PATH"),", thus making the ",(0,o.kt)("inlineCode",{parentName:"p"},"remiz")," command available everywhere.\nYou could also make a symbolic link to the binary from this folder :"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre"},"ln -s ~/remiz/remiz /usr/local/bin/remiz\n")),(0,o.kt)("p",null,"This is the ",(0,o.kt)("strong",{parentName:"p"},"prefered")," way to set up remiz properly since you will need\nto write the configuration where the binary resides."),(0,o.kt)("h2",{id:"test-your-installation"},"Test your installation"),(0,o.kt)("p",null,"If everything is set up, you will be able to execute the ",(0,o.kt)("inlineCode",{parentName:"p"},"remiz")," command."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-text",metastring:'title="remiz"',title:'"remiz"'},"remiz 0.0.17\nPackaging tool to build and deploy packages\n\nUSAGE:\n    remiz [OPTIONS] <SUBCOMMAND>\n\nOPTIONS:\n    -h, --help       Print help information\n    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)\n    -V, --version    Print version information\n\nSUBCOMMANDS:\n    build      Build a package file from a configuration file\n    deploy     Deploy a project from a package file\n    diff       Compare the content of two package files. The output will be a list of metadata &\n               subpackages that are different. More info are displayed if subpackager implements\n               the `info` argument\n    help       Print this message or the help of the given subcommand(s)\n    inspect    Inspect a package file\n    unpack     Unpack a package file into a folder (to inspect it for example)\n")))}m.isMDXComponent=!0}}]);