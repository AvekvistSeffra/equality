var navHeight = 0;
var nav = 0;

var navString = `
		<nav id="outer-nav">
			<div id="nav-handle">
				<div id="nav-button-bar1">
				</div>
				<div id="nav-button-bar2">
				</div>
				<div id="nav-button-bar3">
				</div>
			</div>
		</nav>

		<nav id="inner-nav">
			<a id="nav-top-button" class="nav-button">top</a>
			<a id="nav-precise-button" class="nav-button">precise</a>
			<a id="nav-performant-button" class="nav-button">performant</a>
			<a id="nav-expressive-button" class="nav-button">expressive</a>
			<a id="nav-about-button" class="nav-button">about</a>
			<a id="nav-links-button" class="nav-button">links</a>
			<a id="nav-bottom-button" class="nav-button">copyright</a>
		</nav>`;

var initialize = function() {
	if(nav == 0)
		nav = $("body").prepend(navString);
	
	$("#nav-handle").css("display", "initial");
	
	if($(window).width() < 640) {
		$("#inner-nav").css("top", $("#outer-nav").height());
		$("#inner-nav").css("right", $(document).width() / 2 - $("#inner-nav").width() / 2);
	}
	
	navHeight = $("#outer-nav").height();
}

var toggleNavBar = function() {
	var height = "60vh";
	if($(window).width() < 640)
		height = "60vh";
	
	if($("#inner-nav").css("height") == "0px") {
		$("#inner-nav").animate({
			"height": height,
		}, 300);
	} else {
		$("#inner-nav").animate({
			"height": 0,
		}, 300);
	}

	$("#nav-button-bar1").toggleClass("active-nav");
	$("#nav-button-bar2").toggleClass("active-nav");
	$("#nav-button-bar3").toggleClass("active-nav");
}

var scrollTo = function(ypos) {
	$("html, body").animate({
		scrollTop: ypos,
	}, 1000);
}

/*
var scrollToTag = function(accessor) {
	scrollTo($(accessor).offset().top);
}
*/

var scrollToSection = function(accessor) {
	scrollTo($(accessor).offset().top - navHeight);
}

$(document).ready(function() {
	initialize();
	
	$("#nav-handle").on('click', function() {
		toggleNavBar();
	});
	
	$(".nav-button").on('click', function() {
		toggleNavBar();
	});
	
	$(window).resize(function() {
		initialize();
	});
	
	$("#precise-button").on('click', function() {
		scrollToSection("#precise");
	});
	
	$("#performant-button").on('click', function() {
		scrollToSection("#performant");
	});
	
	$("#expressive-button").on('click', function() {
		scrollToSection("#expressive");
	});
	
	$("#nav-top-button").on('click', function() {
		scrollTo(0);
	});
	
	$("#nav-precise-button").on('click', function() {
		scrollToSection("#precise");
	});
	
	$("#nav-performant-button").on('click', function() {
		scrollToSection("#performant");
	});
	
	$("#nav-expressive-button").on('click', function() {
		scrollToSection("#expressive");
	});
	
	$("#nav-about-button").on('click', function() {
		scrollToSection("#about");
	});
	
	$("#nav-links-button").on('click', function() {
		scrollToSection("#links");
	});
	
	$("#nav-bottom-button").on('click', function() {
		scrollTo($(document).height() - $(window).height());
	});
});
