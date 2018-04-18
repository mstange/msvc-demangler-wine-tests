/* Unit test suite for msvcrt C++ objects
 *
 * Copyright 2003 Jon Griffiths
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301, USA
 */

extern crate msvc_demangler;

#[cfg(test)]
mod tests {
    use ::msvc_demangler::{Result, demangle, DemangleFlags};

    fn expect(input: &str, reference: &str) {
        let demangled = demangle(input, DemangleFlags::LotsOfWhitespace);
        assert_eq!(demangled, Ok(String::from(reference)));
    }

    #[test]
    fn wine_tests() {

        expect("?f@@YAHQBH@Z", "int f(int const * const)");
        expect("?g@@YAHQAY0EA@$$CBH@Z", "int g(int const (* const)[64])");

        expect(
            "??0Klass@std@@AEAA@AEBV01@@Z",
            "std::Klass::Klass(class std::Klass const &)",
        );
        expect("??0?$Klass@V?$Mass@_N@@@std@@QEAA@AEBV01@@Z",
               "std::Klass<class Mass<bool> >::Klass<class Mass<bool> >(class std::Klass<class Mass<bool> > const &)");
        expect("??$ccccc@PAVaaa@@@bar@bb@foo@@DGPAV0@PAV0@PAVee@@IPAPAVaaa@@1@Z",
               "class bar *foo::bb::bar::ccccc<class aaa *>(class bar *,class ee *,unsigned int,class aaa * *,class ee *)");
        expect(
            "??0?$Klass@_N@std@@QEAA@AEBV01@@Z",
            "std::Klass<bool>::Klass<bool>(class std::Klass<bool> const &)",
        );
        expect(
            "??0bad_alloc@std@@QAE@ABV01@@Z",
            "std::bad_alloc::bad_alloc(class std::bad_alloc const &)",
        );
        expect(
            "??0bad_alloc@std@@QAE@PBD@Z",
            "std::bad_alloc::bad_alloc(char const *)",
        );
        expect(
            "??0bad_cast@@AAE@PBQBD@Z",
            "bad_cast::bad_cast(char const * const *)",
        );
        expect(
            "??0bad_cast@@QAE@ABQBD@Z",
            "bad_cast::bad_cast(char const * const &)",
        );
        expect(
            "??0bad_cast@@QAE@ABV0@@Z",
            "bad_cast::bad_cast(class bad_cast const &)",
        );
        expect(
            "??0bad_exception@std@@QAE@ABV01@@Z",
            "std::bad_exception::bad_exception(class std::bad_exception const &)",
        );
        expect(
            "??0bad_exception@std@@QAE@PBD@Z",
            "std::bad_exception::bad_exception(char const *)",
        );
        expect(
            "??0bad_exception@std@@QAE@PBD@Z",
            "std::bad_exception::bad_exception(char const *)",
        );
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@ABV01@@Z",
            "std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(class std::basic_filebuf<char,struct std::char_traits<char> > const &)");
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@ABV01@@Z",
            "std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(class std::basic_filebuf<char,struct std::char_traits<char> > const &)");
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@PAU_iobuf@@@Z",
              "std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(struct _iobuf *)");
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@W4_Uninitialized@1@@Z",
            "std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(enum std::_Uninitialized)");
        expect("??0?$basic_filebuf@GU?$char_traits@G@std@@@std@@QAE@ABV01@@Z",
            "std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >(class std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> > const &)");
        expect("??0?$basic_filebuf@GU?$char_traits@G@std@@@std@@QAE@PAU_iobuf@@@Z",
              "std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >(struct _iobuf *)");
        expect("??0?$basic_filebuf@GU?$char_traits@G@std@@@std@@QAE@W4_Uninitialized@1@@Z",
            "std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >(enum std::_Uninitialized)");
        expect("??0?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAE@ABV01@@Z",
            "std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >(class std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        expect("??0?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAE@ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@H@Z",
            "std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &,int)");

        expect("??0?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAE@H@Z",
              "std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >(int)");
        expect("??0?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAE@ABV01@@Z",
            "std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >(class std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");
        expect("??0?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAE@ABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@1@H@Z",
            "std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &,int)");
        expect("??0?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAE@H@Z",
              "std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >(int)");
        expect("??0?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QAE@ABV_Locinfo@1@I@Z",
            "std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >(class std::_Locinfo const &,unsigned int)");
        expect("??0?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QAE@I@Z",
              "std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >(unsigned int)");
        expect("??0?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@ABV_Locinfo@1@I@Z",
            "std::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >(class std::_Locinfo const &,unsigned int)");
        expect("??0?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@I@Z",
              "std::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >(unsigned int)");
        expect(
            "??0streambuf@@QAE@ABV0@@Z",
            "streambuf::streambuf(class streambuf const &)",
        );
        expect(
            "??0strstreambuf@@QAE@ABV0@@Z",
            "strstreambuf::strstreambuf(class strstreambuf const &)",
        );
        expect(
            "??0strstreambuf@@QAE@H@Z",
            "strstreambuf::strstreambuf(int)",
        );
        expect(
            "??0strstreambuf@@QAE@P6APAXJ@ZP6AXPAX@Z@Z",
            "strstreambuf::strstreambuf(void * (*)(long),void (*)(void *))",
        );
        expect(
            "??0strstreambuf@@QAE@PADH0@Z",
            "strstreambuf::strstreambuf(char *,int,char *)",
        );
        expect(
            "??0strstreambuf@@QAE@PAEH0@Z",
            "strstreambuf::strstreambuf(unsigned char *,int,unsigned char *)",
        );
        expect(
            "??0strstreambuf@@QAE@XZ",
            "strstreambuf::strstreambuf(void)",
        );
        expect(
            "??1__non_rtti_object@std@@UAE@XZ",
            "std::__non_rtti_object::~__non_rtti_object(void)",
        );
        expect(
            "??1__non_rtti_object@@UAE@XZ",
            "__non_rtti_object::~__non_rtti_object(void)",
        );
        expect("??1?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@UAE@XZ",
              "std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::~num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >(void)");
        expect("??1?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@UAE@XZ",
              "std::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >::~num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >(void)");
        expect("??4istream_withassign@@QAEAAV0@ABV0@@Z",
              "class istream_withassign & istream_withassign::operator=(class istream_withassign const &)");
        expect(
            "??4istream_withassign@@QAEAAVistream@@ABV1@@Z",
            "class istream & istream_withassign::operator=(class istream const &)",
        );
        expect(
            "??4istream_withassign@@QAEAAVistream@@PAVstreambuf@@@Z",
            "class istream & istream_withassign::operator=(class streambuf *)",
        );
        expect("??5std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@0@AAV10@AAC@Z",
              "class std::basic_istream<char,struct std::char_traits<char> > & std::operator>>(class std::basic_istream<char,struct std::char_traits<char> > &,signed char &)");
        expect("??5std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@0@AAV10@AAD@Z",
              "class std::basic_istream<char,struct std::char_traits<char> > & std::operator>>(class std::basic_istream<char,struct std::char_traits<char> > &,char &)");
        expect("??5std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@0@AAV10@AAE@Z",
              "class std::basic_istream<char,struct std::char_traits<char> > & std::operator>>(class std::basic_istream<char,struct std::char_traits<char> > &,unsigned char &)");
        expect("??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QAEAAV01@P6AAAVios_base@1@AAV21@@Z@Z",
              "class std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> > & std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >::operator<<(class std::ios_base & (*)(class std::ios_base &))");
        expect("??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QAEAAV01@PAV?$basic_streambuf@GU?$char_traits@G@std@@@1@@Z",
              "class std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> > & std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >::operator<<(class std::basic_streambuf<unsigned short,struct std::char_traits<unsigned short> > *)");
        expect("??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QAEAAV01@PBX@Z",
              "class std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> > & std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >::operator<<(void const *)");

        expect("??_8?$basic_fstream@DU?$char_traits@D@std@@@std@@7B?$basic_ostream@DU?$char_traits@D@std@@@1@@",
              "const std::basic_fstream<char,struct std::char_traits<char> >::`vbtable'{for `std::basic_ostream<char,struct std::char_traits<char> >'}");

        expect("??_8?$basic_fstream@GU?$char_traits@G@std@@@std@@7B?$basic_istream@GU?$char_traits@G@std@@@1@@",
              "const std::basic_fstream<unsigned short,struct std::char_traits<unsigned short> >::`vbtable'{for `std::basic_istream<unsigned short,struct std::char_traits<unsigned short> >'}");
        expect("??_8?$basic_fstream@GU?$char_traits@G@std@@@std@@7B?$basic_ostream@GU?$char_traits@G@std@@@1@@",
              "const std::basic_fstream<unsigned short,struct std::char_traits<unsigned short> >::`vbtable'{for `std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >'}");
        expect("??9std@@YA_NPBDABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@@Z",
              "bool std::operator!=(char const *,class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");
        expect("??9std@@YA_NPBGABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@0@@Z",
              "bool std::operator!=(unsigned short const *,class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");
        expect("??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEAADI@Z",
              "char & std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> >::operator[](unsigned int)");
        expect("??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBEABDI@Z",
              "char const & std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> >::operator[](unsigned int)const ");
        expect("??A?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEAAGI@Z",
              "unsigned short & std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::operator[](unsigned int)");
        expect("??A?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBEABGI@Z",
              "unsigned short const & std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::operator[](unsigned int)const ");
        expect(
            "?abs@std@@YAMABV?$complex@M@1@@Z",
            "float std::abs(class std::complex<float> const &)",
        );
        expect(
            "?abs@std@@YANABV?$complex@N@1@@Z",
            "double std::abs(class std::complex<double> const &)",
        );
        expect(
            "?abs@std@@YAOABV?$complex@O@1@@Z",
            "long double std::abs(class std::complex<long double> const &)",
        );
        expect(
            "?cin@std@@3V?$basic_istream@DU?$char_traits@D@std@@@1@A",
            "class std::basic_istream<char,struct std::char_traits<char> > std::cin",
        );
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAG@Z",
              "class std::istreambuf_iterator<char,struct std::char_traits<char> > std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,unsigned short &)const ");
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAI@Z",
              "class std::istreambuf_iterator<char,struct std::char_traits<char> > std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,unsigned int &)const ");
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAJ@Z",
              "class std::istreambuf_iterator<char,struct std::char_traits<char> > std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,long &)const ");
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAK@Z",
              "class std::istreambuf_iterator<char,struct std::char_traits<char> > std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,unsigned long &)const ");
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAM@Z",
              "class std::istreambuf_iterator<char,struct std::char_traits<char> > std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,float &)const ");

        expect(
            "?_query_new_handler@@YAP6AHI@ZXZ",
            "int (*_query_new_handler(void))(unsigned int)",
        );
        expect("?register_callback@ios_base@std@@QAEXP6AXW4event@12@AAV12@H@ZH@Z",
              "void std::ios_base::register_callback(void (*)(enum std::ios_base::event,class std::ios_base &,int),int)");
        expect("?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QAEAAV12@JW4seekdir@ios_base@2@@Z",
              "class std::basic_istream<char,struct std::char_traits<char> > & std::basic_istream<char,struct std::char_traits<char> >::seekg(long,enum std::ios_base::seekdir)");
        expect("?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QAEAAV12@V?$fpos@H@2@@Z",
              "class std::basic_istream<char,struct std::char_traits<char> > & std::basic_istream<char,struct std::char_traits<char> >::seekg(class std::fpos<int>)");
        expect("?seekg@?$basic_istream@GU?$char_traits@G@std@@@std@@QAEAAV12@JW4seekdir@ios_base@2@@Z",
              "class std::basic_istream<unsigned short,struct std::char_traits<unsigned short> > & std::basic_istream<unsigned short,struct std::char_traits<unsigned short> >::seekg(long,enum std::ios_base::seekdir)");
        expect("?seekg@?$basic_istream@GU?$char_traits@G@std@@@std@@QAEAAV12@V?$fpos@H@2@@Z",
              "class std::basic_istream<unsigned short,struct std::char_traits<unsigned short> > & std::basic_istream<unsigned short,struct std::char_traits<unsigned short> >::seekg(class std::fpos<int>)");
        expect("?seekoff@?$basic_filebuf@DU?$char_traits@D@std@@@std@@MAE?AV?$fpos@H@2@JW4seekdir@ios_base@2@H@Z",
              "class std::fpos<int> std::basic_filebuf<char,struct std::char_traits<char> >::seekoff(long,enum std::ios_base::seekdir,int)");
        expect("?seekoff@?$basic_filebuf@GU?$char_traits@G@std@@@std@@MAE?AV?$fpos@H@2@JW4seekdir@ios_base@2@H@Z",
              "class std::fpos<int> std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::seekoff(long,enum std::ios_base::seekdir,int)");
        expect(
            "?set_new_handler@@YAP6AXXZP6AXXZ@Z",
            "void (*set_new_handler(void (*)(void)))(void)",
        );
        expect("?str@?$basic_istringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z",
              "void std::basic_istringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");
        expect("?str@?$basic_istringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ",
              "class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > std::basic_istringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");
        expect("?str@?$basic_istringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z",
              "void std::basic_istringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");
        expect("?str@?$basic_istringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ",
              "class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > std::basic_istringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");
        expect("?str@?$basic_ostringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z",
              "void std::basic_ostringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");
        expect("?str@?$basic_ostringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ",
              "class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > std::basic_ostringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");
        expect("?str@?$basic_ostringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z",
              "void std::basic_ostringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");
        expect("?str@?$basic_ostringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ",
              "class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > std::basic_ostringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");
        expect("?str@?$basic_stringbuf@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z",
              "void std::basic_stringbuf<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");
        expect("?str@?$basic_stringbuf@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ",
              "class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > std::basic_stringbuf<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");
        expect("?str@?$basic_stringbuf@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z",
              "void std::basic_stringbuf<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");
        expect("?str@?$basic_stringbuf@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ",
              "class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > std::basic_stringbuf<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");
        expect("?str@?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z",
              "void std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");
        expect("?str@?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ",
              "class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");
        expect("?str@?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z",
              "void std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");
        expect("?str@?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ",
              "class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");
        expect("?_Sync@ios_base@std@@0_NA", "bool std::ios_base::_Sync");
        expect("??_U@YAPAXI@Z", "void * operator new[](unsigned int)");
        expect("??_V@YAXPAX@Z", "void operator delete[](void *)");
        expect("??X?$_Complex_base@M@std@@QAEAAV01@ABM@Z",
              "class std::_Complex_base<float> & std::_Complex_base<float>::operator*=(float const &)");
        expect("??Xstd@@YAAAV?$complex@M@0@AAV10@ABV10@@Z",
              "class std::complex<float> & std::operator*=(class std::complex<float> &,class std::complex<float> const &)");
        expect("?aaa@@YAHAAUbbb@@@Z", "int aaa(struct bbb &)");
        expect("?aaa@@YAHBAUbbb@@@Z", "int aaa(struct bbb & volatile)");
        expect("?aaa@@YAHPAUbbb@@@Z", "int aaa(struct bbb *)");
        expect("?aaa@@YAHQAUbbb@@@Z", "int aaa(struct bbb * const)");
        expect("?aaa@@YAHRAUbbb@@@Z", "int aaa(struct bbb * volatile)");
        expect(
            "?aaa@@YAHSAUbbb@@@Z",
            "int aaa(struct bbb * const volatile)",
        );
        // expect("??0aa.a@@QAE@XZ", "??0aa.a@@QAE@XZ"); // This should fail to demangle, probably because of the period.
        expect("??0aa$_3a@@QAE@XZ", "aa$_3a::aa$_3a(void)");
        expect("??2?$aaa@AAUbbb@@AAUccc@@AAU2@@ddd@1eee@2@QAEHXZ",
              "int eee::eee::ddd::ddd::aaa<struct bbb &,struct ccc &,struct ccc &>::operator new(void)");

        expect("?pSW@@3P6GHKPAX0PAU_tagSTACKFRAME@@0P6GH0K0KPAK@ZP6GPAX0K@ZP6GK0K@ZP6GK00PAU_tagADDRESS@@@Z@ZA",
              "int (*pSW)(unsigned long,void *,void *,struct _tagSTACKFRAME *,void *,int (*)(void *,unsigned long,void *,unsigned long,unsigned long *),void * (*)(void *,unsigned long),unsigned long (*)(void *,unsigned long),unsigned long (*)(void *,void *,struct _tagADDRESS *))");

        expect("?$_aaa@Vbbb@@", "_aaa<class bbb>");
        expect(
            "?$aaa@Vbbb@ccc@@Vddd@2@",
            "aaa<class ccc::bbb,class ccc::ddd>",
        );
        expect(
            "??0?$Foo@P6GHPAX0@Z@@QAE@PAD@Z",
            "Foo<int (*)(void *,void *)>::Foo<int (*)(void *,void *)>(char *)",
        );
        expect(
            "??0?$Foo@P6GHPAX0@Z@@QAE@PAD@Z",
            "Foo<int (*)(void *,void *)>::Foo<int (*)(void *,void *)>(char *)",
        );
        expect(
            "?Qux@Bar@@0PAP6AHPAV1@AAH1PAH@ZA",
            "int (* *Bar::Qux)(class Bar *,int &,int &,int *)",
        );
        // expect("?Qux@Bar@@0PAP6AHPAV1@AAH1PAH@ZA", "Bar::Qux"); // ???
        expect("?$AAA@$DBAB@", "AAA<`template-parameter257'>");
        expect("?$AAA@?C@", "AAA<`template-parameter-2'>");
        expect("?$AAA@PAUBBB@@", "AAA<struct BBB *>");
        expect("??$ccccc@PAVaaa@@@bar@bb@foo@@DGPAV0@PAV0@PAVee@@IPAPAVaaa@@1@Z",
            "class bar *foo::bb::bar::ccccc<class aaa *>(class bar *,class ee *,unsigned int,class aaa * *,class ee *)");

        /* this isn't real c++
        expect(
            "?f@T@@QAEHQCY1BE@BO@D@Z",
            "int T::f(char (volatile * const)[20][30])",
        );
        expect(
            "?f@T@@QAEHQAY2BE@BO@CI@D@Z",
            "int T::f(char (* const)[20][30][40])",
        );
        expect(
            "?f@T@@QAEHQAY1BE@BO@$$CBD@Z",
            "int T::f(char const (* const)[20][30])",
        );*/
        expect("??0?$Foo@U?$vector_c@H$00$01$0?1$0A@$0A@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@@mpl@boost@@@@QAE@XZ",
              "Foo<struct boost::mpl::vector_c<int,1,2,-2,0,0,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647> >::Foo<struct boost::mpl::vector_c<int,1,2,-2,0,0,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647> >(void)");
        expect(
            "?swprintf@@YAHPAGIPBGZZ",
            "int swprintf(unsigned short *,unsigned int,unsigned short const *,...)",
        );
        expect(
            "?vswprintf@@YAHPAGIPBGPAD@Z",
            "int vswprintf(unsigned short *,unsigned int,unsigned short const *,char *)",
        );
        expect(
            "?vswprintf@@YAHPA_WIPB_WPAD@Z",
            "int vswprintf(wchar_t *,unsigned int,wchar_t const *,char *)",
        );
        expect(
            "?swprintf@@YAHPA_WIPB_WZZ",
            "int swprintf(wchar_t *,unsigned int,wchar_t const *,...)",
        );
        expect("??Xstd@@YAAEAV?$complex@M@0@AEAV10@AEBV10@@Z",
              "class std::complex<float> & std::operator*=(class std::complex<float> &,class std::complex<float> const &)");
        expect(
            "?_Doraise@bad_cast@std@@MEBAXXZ",
            "void std::bad_cast::_Doraise(void)const ",
        );
        expect("??$?DM@std@@YA?AV?$complex@M@0@ABMABV10@@Z",
            "class std::complex<float> std::operator*<float>(float const &,class std::complex<float> const &)");
        /*expect("?_R2@?BN@???$_Fabs@N@std@@YANAEBV?$complex@N@1@PEAH@Z@4NB",
            "double const `double std::_Fabs<double>(class std::complex<double> const &,int *)'::`29'::_R2");
        expect(
            "?vtordisp_thunk@std@@$4PPPPPPPM@3EAA_NXZ",
            "[thunk]:bool std::vtordisp_thunk`vtordisp{4294967292,4}' (void)",
        );
        expect(
            "??_9CView@@$BBII@AE",
            "[thunk]: CView::`vcall'{392,{flat}}' }'",
        );
        expect("?_dispatch@_impl_Engine@SalomeApp@@$R4CE@BA@PPPPPPPM@7AE_NAAVomniCallHandle@@@Z",
            "[thunk]:bool SalomeApp::_impl_Engine::_dispatch`vtordispex{36,16,4294967292,8}' (class omniCallHandle &)");
        expect(
            "?_Doraise@bad_cast@std@@MEBAXXZ",
            "void std::bad_cast::_Doraise(void) ",
        );*/
        expect("??Xstd@@YAAEAV?$complex@M@0@AEAV10@AEBV10@@Z",
              "class std::complex<float> & std::operator*=(class std::complex<float> &,class std::complex<float> const &)");
        expect("??Xstd@@YAAEAV?$complex@M@0@AEAV10@AEBV10@@Z",
            "class std::complex<float> & std::operator*=(class std::complex<float> &,class std::complex<float> const &)");
        expect("??$run@XVTask_Render_Preview@@@QtConcurrent@@YA?AV?$QFuture@X@@PEAVTask_Render_Preview@@P82@EAAXXZ@Z",
            "class QFuture<void> QtConcurrent::run<void,class Task_Render_Preview>(class Task_Render_Preview *,void (Task_Render_Preview::*)(void))");
        expect(
            "??_E?$TStrArray@$$BY0BAA@D$0BA@@@UAEPAXI@Z",
            "void * TStrArray<char [256],16>::`vector deleting destructor'(unsigned int)",
        );
    }
}
