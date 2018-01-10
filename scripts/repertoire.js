function new_song() {
  if (!document.getElementById('edit-song').hidden) {
    try_cancel_edit_song();
  } else if (document.getElementById('new-song').hidden) {
    document.getElementById('song-details').hidden = true;
    document.getElementById('new-song').hidden = false;
  }
}

// TODO: FIX THIS
try_cancel_edit_song() {
  $.confirm({
      title: 'Confirm!',
      content: 'Simple confirm!',
      buttons: {
          confirm: function () {
              $.alert('Confirmed!');
          },
          cancel: function () {
              $.alert('Canceled!');
          },
          somethingElse: {
              text: 'Something else',
              btnClass: 'btn-blue',
              keys: ['enter', 'shift'],
              action: function(){
                  $.alert('Something else?');
              }
          }
      }
  });
}

function load_song_details(song_id) {
  if (!document.getElementById('song-details').hidden) {
    // console.log(song_id);
    $.get(
      '/song/' + song_id,
      function(data) {
        // console.log(data);
        $('div[id=song-details]').replaceWith(data);
      },
      'html'
    );
  }
}
