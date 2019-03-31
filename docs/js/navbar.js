// Initializing the height of the nav
// as well as the reference to it. 
var navHeight = 0;
var nav = 0;

// The entire HTML for the navbar.
// If JavaScript is deactivated, it shouldn't
// exist. 
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
			<a id="nav-bottom-button" class="nav-button">copyright</a>
		</nav>`;

// Function to initialize (and reinitialize) the navbar. 
var initialize = function() {
	// Adds a new nav if it doesn't exist. 
	if(nav == 0)
		nav = $("body").prepend(navString);

	// Sets the display mode on the handle to initial. 
	$("#nav-handle").css("display", "initial");
	
	// Sets the top position of the navbar.  
	$("#inner-nav").css("top", $("#outer-nav").height());
	
	// Checks the window width and initializes the 
	// inner navbar correctly. Only for larger
	// screebs
	if($(window).width() > 640) {
		$("#inner-nav").css("right", $(document).width() / 2 - $("#inner-nav").width() / 2);
	}

	// Sets the height of the navbar. 
	navHeight = $("#outer-nav").height();
}

// Function that toggles the navbar. 
var toggleNavBar = function() {
	// Depending on the size of the window
	// or the size of the device used, 
	// the height of the navbar changes. 
	var height = "60vh";
	if($(window).width() < 640)
		height = "100vh";
	
	// Depending on the height of the navbar,
	// it sets the height and animation. 
	if($("#inner-nav").css("height") == "0px") {
		$("#inner-nav").animate({
			"height": height,
		}, 300);
	} else {
		$("#inner-nav").animate({
			"height": 0,
		}, 300);
	}

	// An attempt to toggle the pieces of bread 
	// at the top of the screen. 
	$("#nav-button-bar1").toggleClass("active-nav");
	$("#nav-button-bar2").toggleClass("active-nav");
	$("#nav-button-bar3").toggleClass("active-nav");
}

// Function to scroll to a specific position
// For smooth scrolling. 
var scrollTo = function(ypos) {
	$("html, body").animate({
		scrollTop: ypos,
	}, 1000);
}

// A small helper function, an abstraction
// that makes it easier to smooth scroll
// to sections. It's also subtracting
// the height of the navbar handle. 
var scrollToSection = function(accessor) {
	scrollTo($(accessor).offset().top - navHeight);
}

// When the document is ready, this runs. 
$(document).ready(function() {
	initialize();
	
	// Binds the on-click callback for the 
	// #nav-handle. 
	$("#nav-handle").on('click', function() {
		toggleNavBar();
	});
	
	// Binds the on-click callback for the
	// .nav-button. 
	$(".nav-button").on('click', function() {
		toggleNavBar();
	});
	
	// Binds the resize callback to the
	// initialize function.
	// This code runs when the window
	// is resized. 
	$(window).resize(function() {
		initialize();
	});
	
	// Clicking the #precise-button scrolls
	// you to the correct section. 
	$("#precise-button").on('click', function() {
		scrollToSection("#precise");
	});
	
	// Clicking the #performant-button scrolls
	// you to the correct section. 
	$("#performant-button").on('click', function() {
		scrollToSection("#performant");
	});
	
	// Clicking the #expressive-button scrolls
	// you to the correct section. 
	$("#expressive-button").on('click', function() {
		scrollToSection("#expressive");
	});
	
	// Clicking the #nav-top-button scrolls
	// you to the top. 
	$("#nav-top-button").on('click', function() {
		scrollTo(0);
	});
	
	// Clicking the #nav-precise-button scrolls
	// you to the correct section. 
	$("#nav-precise-button").on('click', function() {
		scrollToSection("#precise");
	});
	
	// Clicking the #nav-performant-button scrolls
	// you to the correct section. 
	$("#nav-performant-button").on('click', function() {
		scrollToSection("#performant");
	});
	
	// Clicking the #nav-expressive-button scrolls
	// you to the correct section. 
	$("#nav-expressive-button").on('click', function() {
		scrollToSection("#expressive");
	});
	
	// Clicking the #nav-about-button scrolls
	// you to the correct section. 
	$("#nav-about-button").on('click', function() {
		scrollToSection("#about");
	});

	// Clicking the #nav-bottom-button scrolls
	// you to the bottom of the document. 
	$("#nav-bottom-button").on('click', function() {
		scrollTo($(document).height() - $(window).height());
	});
});
