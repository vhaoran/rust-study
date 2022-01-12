----------------------------
$("p")
所有 <p> 元素都隐藏
----------------------------
$("#test")
id="test" 属性的元素
----------------------------
$(".test")
class="test"
----------------------------
*	$("*")	所有元素
#id	$("#lastname")	id="lastname" 的元素
.class	$(".intro")	class="intro" 的所有元素
.class,.class	$(".intro,.demo")	class 为 "intro" 或 "demo" 的所有元素
element	$("p")	所有 <p> 元素
el1,el2,el3	$("h1,div,p")	所有 <h1>、<div> 和 <p> 元素
:first	$("p:first")	第一个 <p> 元素
:last	$("p:last")	最后一个 <p> 元素
:even	$("tr:even")	所有偶数 <tr> 元素，索引值从 0 开始，第一个元素是偶数 (0)，第二个元素是奇数 (1)，以此类推。
:odd	$("tr:odd")	所有奇数 <tr> 元素，索引值从 0 开始，第一个元素是偶数 (0)，第二个元素是奇数 (1)，以此类推。
:first-child	$("p:first-child")	属于其父元素的第一个子元素的所有 <p> 元素
:first-of-type	$("p:first-of-type")	属于其父元素的第一个 <p> 元素的所有 <p> 元素
:last-child	$("p:last-child")	属于其父元素的最后一个子元素的所有 <p> 元素
:last-of-type	$("p:last-of-type")	属于其父元素的最后一个 <p> 元素的所有 <p> 元素
:nth-child(n)	$("p:nth-child(2)")	属于其父元素的第二个子元素的所有 <p> 元素
:nth-last-child(n)	$("p:nth-last-child(2)")	属于其父元素的第二个子元素的所有 <p> 元素，从最后一个子元素开始计数
:nth-of-type(n)	$("p:nth-of-type(2)")	属于其父元素的第二个 <p> 元素的所有 <p> 元素
:nth-last-of-type(n)	$("p:nth-last-of-type(2)")	属于其父元素的第二个 <p> 元素的所有 <p> 元素，从最后一个子元素开始计数
:only-child	$("p:only-child")	属于其父元素的唯一子元素的所有 <p> 元素
:only-of-type	$("p:only-of-type")	属于其父元素的特定类型的唯一子元素的所有 <p> 元素
parent > child	$("div > p")	<div> 元素的直接子元素的所有 <p> 元素
parent descendant	$("div p")	<div> 元素的后代的所有 <p> 元素
element + next	$("div + p")	每个 <div> 元素相邻的下一个 <p> 元素
element ~ siblings	$("div ~ p")	<div> 元素同级的所有 <p> 元素
:eq(index)	$("ul li:eq(3)")	列表中的第四个元素（index 值从 0 开始）
:gt(no)	$("ul li:gt(3)")	列举 index 大于 3 的元素
:lt(no)	$("ul li:lt(3)")	列举 index 小于 3 的元素
:not(selector)	$("input:not(:empty)")	所有不为空的输入元素
:header	$(":header")	所有标题元素 <h1>, <h2> ...
:animated	$(":animated")	所有动画元素
:focus	$(":focus")	当前具有焦点的元素
:contains(text)	$(":contains('Hello')")	所有包含文本 "Hello" 的元素
:has(selector)	$("div:has(p)")	所有包含有 <p> 元素在其内的 <div> 元素
:empty	$(":empty")	所有空元素
:parent	$(":parent")	匹配所有含有子元素或者文本的父元素。
:hidden	$("p:hidden")	所有隐藏的 <p> 元素
:visible	$("table:visible")	所有可见的表格
:root	$(":root")	文档的根元素
:lang(language)	$("p:lang(de)")	所有 lang 属性值为 "de" 的 <p> 元素

[attribute]	$("[href]")	所有带有 href 属性的元素
[attribute=value]	$("[href='default.htm']")	所有带有 href 属性且值等于 "default.htm" 的元素
[attribute!=value]	$("[href!='default.htm']")	所有带有 href 属性且值不等于 "default.htm" 的元素
[attribute$=value]	$("[href$='.jpg']")	所有带有 href 属性且值以 ".jpg" 结尾的元素
[attribute|=value]	$("[title|='Tomorrow']")	所有带有 title 属性且值等于 'Tomorrow' 或者以 'Tomorrow' 后跟连接符作为开头的字符串
[attribute^=value]	$("[title^='Tom']")	所有带有 title 属性且值以 "Tom" 开头的元素
[attribute~=value]	$("[title~='hello']")	所有带有 title 属性且值包含单词 "hello" 的元素
[attribute*=value]	$("[title*='hello']")	所有带有 title 属性且值包含字符串 "hello" 的元素
[name=value][name2=value2]	$( "input[id][name$='man']" )	带有 id 属性，并且 name 属性以 man 结尾的输入框
----------------------------
.基本选择器
$("#id")            //ID选择器
$("div")            //元素选择器
$(".classname")     //类选择器
$(".classname,.classname1,#id1")     //组合选择器
----------------------------
层次选择器
$("#id>.classname ")    //子元素选择器
$("#id .classname ")    //后代元素选择器
$("#id + .classname ")    //紧邻下一个元素选择器
$("#id ~ .classname ")    //兄弟元素选择器
----------------------------
过滤选择器(重点)
$("li:first")    //第一个li
$("li:last")     //最后一个li
$("li:even")     //挑选下标为偶数的li
$("li:odd")      //挑选下标为奇数的li
$("li:eq(4)")    //下标等于 4 的li(第五个 li 元素)
$("li:gt(2)")    //下标大于 2 的li
$("li:lt(2)")    //下标小于 2 的li
$("li:not(#runoob)") //挑选除 id="runoob" 以外的所有li
----------------------------
内容过滤选择器

$("div:contains('Runob')")    // 包含 Runob文本的元素
$("td:empty")                 //不包含子元素或者文本的空元素
$("div:has(selector)")        //含有选择器所匹配的元素
$("td:parent")                //含有子元素或者文本的元素
----------------------------
可见性过滤选择器
$("li:hidden")       //匹配所有不可见元素，或type为hidden的元素
$("li:visible")      //匹配所有可见元素
----------------------------
属性过滤选择器
$("div[id]")        //所有含有 id 属性的 div 元素
$("div[id='123']")        // id属性值为123的div 元素
$("div[id!='123']")        // id属性值不等于123的div 元素
$("div[id^='qq']")        // id属性值以qq开头的div 元素
$("div[id$='zz']")        // id属性值以zz结尾的div 元素
$("div[id*='bb']")        // id属性值包含bb的div 元素
$("input[id][name$='man']") //多属性选过滤，同时满足两个属性的条件的元素
----------------------------
状态过滤选择器
$("input:enabled")    // 匹配可用的 input
$("input:disabled")   // 匹配不可用的 input
$("input:checked")    // 匹配选中的 input
$("option:selected")  // 匹配选中的 option
----------------------------
表单选择器
$(":input")      //匹配所有 input, textarea, select 和 button 元素
$(":text")       //所有的单行文本框，$(":text") 等价于$("[type=text]")，推荐使用$("input:text")效率更高，下同
$(":password")   //所有密码框
$(":radio")      //所有单选按钮
$(":checkbox")   //所有复选框
$(":submit")     //所有提交按钮
$(":reset")      //所有重置按钮
$(":button")     //所有button按钮
$(":file")       //所有文件域
----------------------------

----------------------------
