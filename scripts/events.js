function load_event_details(event_id) {
  // console.log(event_id);
  $.get(
    '/event/' + event_id,
    function(data) {
      // console.log(data);
      $('div[id=event-details]').replaceWith(data);
    },
    'html'
  );
}

function new_event() {
  document.getElementById('event-details').hidden = true;
  document.getElementById('new-event').hidden = false;
}

function submit_new_event() {
  var error_count = 0;
  if (document.getElementById('title').value.length == 0) {
    error_count++;
    $('span[id="title-error"]').show();
  } else {
    $('span[id="title-error"]').hide();
  }
  if (document.getElementById('location').value.length == 0) {
    error_count++;
    $('span[id="location-error"]').show();
  } else {
    $('span[id="location-error"]').hide();
  }
  if (document.getElementById('start_date').value.length < 10 ||
      document.getElementById('start_time').value.length < 5) {
    error_count++;
    $('span[id="start-error"]').show();
  } else {
    $('span[id="start-error"]').hide();
  }
  if (document.getElementById('end_date').value.length < 10 ||
      document.getElementById('end_time').value.length < 5) {
    error_count++;
    $('span[id="end-error"]').show();
  } else {
    $('span[id="end-error"]').hide();
  }

  if (error_count == 0) {
    $.ajax({
      url:'/events',
      type:'post',
      data:$('form[id="new_event_form"]').serialize()
    });
  }
}

function load_attendance(event_id) {
  // console.log(event_id);
  $.get(
    '/attendance/' + event_id,
    function(data) {
      // console.log(data);
      $('div[id=event-details]').replaceWith(data);
    },
    'html'
  );
}

function update_attendance(attendance_id) {
  var form = document.getElementById('attendance-form');
  document.getElementById('should_attend').value = document.getElementById('should_attend-' + attendance_id).checked;
  document.getElementById('did_attend').value = document.getElementById('did_attend-' + attendance_id).checked;
  document.getElementById('minutes_late').value = document.getElementById('minutes_late-' + attendance_id).value;
  document.getElementById('confirmed').value = document.getElementById('confirmed-' + attendance_id).checked;
  $.ajax({
    url:'/attendance/' + attendance_id,
    type:'post',
    data:$('form[id="attendance-form"]').serialize(),
    success: function(returned) {
      if (document.getElementById('did_attend-' + attendance_id).checked) {
        $('td[id="attended-' + attendance_id + '"]').text('Yes').css({'color': 'green'});
      } else {
        $('td[id="attended-' + attendance_id + '"]').text('No').css({'color': 'red'});
      }
    }
  });
}

function load_song_details(song_id) {
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
