$(document).ready(function() {
    var re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    $("#email-form").keyup(function() {
        if($("#email-form").val().length > 0) {
            var correct = re.exec($("#email-form").val());
            if(!correct) {
                $("#validator-text").text("Enter a correct email address");
            } else {
                $("#validator-text").text("Awesome, just hit 'Join' and start receiving emails. ");
            }
        } else {
            $("#validator-text").text("");
        }
    })
});
