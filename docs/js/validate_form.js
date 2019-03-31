// Execute this code when the entire site
// has finished loading. 
$(document).ready(function() {
    // Initialize the regular expression. 
    var re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    
    // When you type in the form, this function
    // is called
    $("#email-form").keyup(function() {
        // If the length of the contents in the form
        // is greater than 0 then
        if($("#email-form").val().length > 0) {

            // check if the content matches the regular
            // expression rules specified above. 
            var correct = re.exec($("#email-form").val());

            // If not, then
            if(!correct) {
                // disable the submit button. 
                $("#email-form-submit").prop("disabled", true);
            } else {
                // Otherwise, enable it. 
                $("#email-form-submit").prop("disabled", false);
            }
        } else {
            // If the content length isn't greater than 0
            // then disable the button. 
            $("#email-form-submit").prop("disabled", true);
        }
    })
});
