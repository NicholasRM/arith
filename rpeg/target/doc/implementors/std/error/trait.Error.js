(function() {var implementors = {};
implementors["crossbeam_channel"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"crossbeam_channel/struct.SendError.html\" title=\"struct crossbeam_channel::SendError\">SendError</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_channel::err::SendError"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"crossbeam_channel/enum.TrySendError.html\" title=\"enum crossbeam_channel::TrySendError\">TrySendError</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_channel::err::TrySendError"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"crossbeam_channel/enum.SendTimeoutError.html\" title=\"enum crossbeam_channel::SendTimeoutError\">SendTimeoutError</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_channel::err::SendTimeoutError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"crossbeam_channel/struct.RecvError.html\" title=\"struct crossbeam_channel::RecvError\">RecvError</a>","synthetic":false,"types":["crossbeam_channel::err::RecvError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"crossbeam_channel/enum.TryRecvError.html\" title=\"enum crossbeam_channel::TryRecvError\">TryRecvError</a>","synthetic":false,"types":["crossbeam_channel::err::TryRecvError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"crossbeam_channel/enum.RecvTimeoutError.html\" title=\"enum crossbeam_channel::RecvTimeoutError\">RecvTimeoutError</a>","synthetic":false,"types":["crossbeam_channel::err::RecvTimeoutError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"crossbeam_channel/struct.TrySelectError.html\" title=\"struct crossbeam_channel::TrySelectError\">TrySelectError</a>","synthetic":false,"types":["crossbeam_channel::err::TrySelectError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"crossbeam_channel/struct.SelectTimeoutError.html\" title=\"struct crossbeam_channel::SelectTimeoutError\">SelectTimeoutError</a>","synthetic":false,"types":["crossbeam_channel::err::SelectTimeoutError"]}];
implementors["gif"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"gif/struct.DecodingFormatError.html\" title=\"struct gif::DecodingFormatError\">DecodingFormatError</a>","synthetic":false,"types":["gif::reader::decoder::DecodingFormatError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"gif/enum.DecodingError.html\" title=\"enum gif::DecodingError\">DecodingError</a>","synthetic":false,"types":["gif::reader::decoder::DecodingError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"gif/enum.EncodingError.html\" title=\"enum gif::EncodingError\">EncodingError</a>","synthetic":false,"types":["gif::encoder::EncodingError"]}];
implementors["png"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"png/enum.DecodingError.html\" title=\"enum png::DecodingError\">DecodingError</a>","synthetic":false,"types":["png::decoder::stream::DecodingError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"png/enum.EncodingError.html\" title=\"enum png::EncodingError\">EncodingError</a>","synthetic":false,"types":["png::encoder::EncodingError"]}];
implementors["weezl"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"weezl/enum.LzwError.html\" title=\"enum weezl::LzwError\">LzwError</a>","synthetic":false,"types":["weezl::error::LzwError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()