
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
    use msvc_demangler::{demangle, DemangleFlags};

    fn expect(input: &str, reference: &str) {
        let demangled = demangle(input, DemangleFlags::LotsOfWhitespace);
        assert_eq!(demangled, Ok(String::from(reference)));
    }

    #[test]
    fn wine_tests() {
        /* 0 */
        expect(
            "??0bad_alloc@std@@QAE@ABV01@@Z",
            "public: __thiscall std::bad_alloc::bad_alloc(class std::bad_alloc const &)",
        );

        /* 1 */
        expect(
            "??0bad_alloc@std@@QAE@PBD@Z",
            "public: __thiscall std::bad_alloc::bad_alloc(char const *)",
        );

        /* 2 */
        expect(
            "??0bad_cast@@AAE@PBQBD@Z",
            "private: __thiscall bad_cast::bad_cast(char const * const *)",
        );

        /* 3 */
        expect(
            "??0bad_cast@@QAE@ABQBD@Z",
            "public: __thiscall bad_cast::bad_cast(char const * const &)",
        );

        /* 4 */
        expect(
            "??0bad_cast@@QAE@ABV0@@Z",
            "public: __thiscall bad_cast::bad_cast(class bad_cast const &)",
        );

        /* 5 */
        expect("??0bad_exception@std@@QAE@ABV01@@Z",
         "public: __thiscall std::bad_exception::bad_exception(class std::bad_exception const &)");

        /* 6 */
        expect(
            "??0bad_exception@std@@QAE@PBD@Z",
            "public: __thiscall std::bad_exception::bad_exception(char const *)",
        );

        /* 7 */
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@ABV01@@Z",
         "public: __thiscall std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(class std::basic_filebuf<char,struct std::char_traits<char> > const &)");

        /* 8 */
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@PAU_iobuf@@@Z",
         "public: __thiscall std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(struct _iobuf *)");

        /* 9 */
        expect("??0?$basic_filebuf@DU?$char_traits@D@std@@@std@@QAE@W4_Uninitialized@1@@Z",
         "public: __thiscall std::basic_filebuf<char,struct std::char_traits<char> >::basic_filebuf<char,struct std::char_traits<char> >(enum std::_Uninitialized)");

        /* 10 */
        expect("??0?$basic_filebuf@GU?$char_traits@G@std@@@std@@QAE@ABV01@@Z",
          "public: __thiscall std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >(class std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> > const &)");

        /* 11 */
        expect("??0?$basic_filebuf@GU?$char_traits@G@std@@@std@@QAE@PAU_iobuf@@@Z",
          "public: __thiscall std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >(struct _iobuf *)");

        /* 12 */
        expect("??0?$basic_filebuf@GU?$char_traits@G@std@@@std@@QAE@W4_Uninitialized@1@@Z",
          "public: __thiscall std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >(enum std::_Uninitialized)");

        /* 13 */
        expect("??0?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAE@ABV01@@Z",
          "public: __thiscall std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >(class std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        /* 14 */
        expect("??0?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAE@ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@H@Z",
          "public: __thiscall std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &,int)");

        /* 15 */
        expect("??0?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAE@H@Z",
          "public: __thiscall std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >(int)");

        /* 16 */
        expect("??0?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAE@ABV01@@Z",
          "public: __thiscall std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >(class std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");

        /* 17 */
        expect("??0?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAE@ABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@1@H@Z",
          "public: __thiscall std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &,int)");

        /* 18 */
        expect("??0?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAE@H@Z",
          "public: __thiscall std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >(int)");

        /* 19 */
        expect("??0?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QAE@ABV_Locinfo@1@I@Z",
          "public: __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >(class std::_Locinfo const &,unsigned int)");

        /* 20 */
        expect("??0?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QAE@I@Z",
          "public: __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >(unsigned int)");

        /* 21 */
        expect("??0?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@ABV_Locinfo@1@I@Z",
          "public: __thiscall std::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >(class std::_Locinfo const &,unsigned int)");

        /* 22 */
        expect("??0?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@I@Z", "public: __thiscall std::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >(unsigned int)");

        /* 23 */
        expect(
            "??0streambuf@@QAE@ABV0@@Z",
            "public: __thiscall streambuf::streambuf(class streambuf const &)",
        );

        /* 24 */
        expect(
            "??0strstreambuf@@QAE@ABV0@@Z",
            "public: __thiscall strstreambuf::strstreambuf(class strstreambuf const &)",
        );

        /* 25 */
        expect(
            "??0strstreambuf@@QAE@H@Z",
            "public: __thiscall strstreambuf::strstreambuf(int)",
        );

        /* 26 */
        // "__cdecl (*" vs "(__cdecl*"
        // expect("??0strstreambuf@@QAE@P6APAXJ@ZP6AXPAX@Z@Z", "public: __thiscall strstreambuf::strstreambuf(void * (__cdecl*)(long),void (__cdecl*)(void *))");

        /* 27 */
        expect(
            "??0strstreambuf@@QAE@PADH0@Z",
            "public: __thiscall strstreambuf::strstreambuf(char *,int,char *)",
        );

        /* 28 */
        expect(
            "??0strstreambuf@@QAE@PAEH0@Z",
            "public: __thiscall strstreambuf::strstreambuf(unsigned char *,int,unsigned char *)",
        );

        /* 29 */
        expect(
            "??0strstreambuf@@QAE@XZ",
            "public: __thiscall strstreambuf::strstreambuf(void)",
        );

        /* 30 */
        expect(
            "??1__non_rtti_object@std@@UAE@XZ",
            "public: virtual __thiscall std::__non_rtti_object::~__non_rtti_object(void)",
        );

        /* 31 */
        expect(
            "??1__non_rtti_object@@UAE@XZ",
            "public: virtual __thiscall __non_rtti_object::~__non_rtti_object(void)",
        );

        /* 32 */
        expect("??1?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@UAE@XZ", "public: virtual __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::~num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >(void)");

        /* 33 */
        expect("??1?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@UAE@XZ", "public: virtual __thiscall std::num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >::~num_get<unsigned short,class std::istreambuf_iterator<unsigned short,struct std::char_traits<unsigned short> > >(void)");

        /* 34 */
        expect("??4istream_withassign@@QAEAAV0@ABV0@@Z", "public: class istream_withassign & __thiscall istream_withassign::operator=(class istream_withassign const &)");

        /* 35 */
        expect("??4istream_withassign@@QAEAAVistream@@ABV1@@Z", "public: class istream & __thiscall istream_withassign::operator=(class istream const &)");

        /* 36 */
        expect(
            "??4istream_withassign@@QAEAAVistream@@PAVstreambuf@@@Z",
            "public: class istream & __thiscall istream_withassign::operator=(class streambuf *)",
        );

        /* 37 */
        expect("??5std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@0@AAV10@AAC@Z", "class std::basic_istream<char,struct std::char_traits<char> > & __cdecl std::operator>>(class std::basic_istream<char,struct std::char_traits<char> > &,signed char &)");

        /* 38 */
        expect("??5std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@0@AAV10@AAD@Z", "class std::basic_istream<char,struct std::char_traits<char> > & __cdecl std::operator>>(class std::basic_istream<char,struct std::char_traits<char> > &,char &)");

        /* 39 */
        expect("??5std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@0@AAV10@AAE@Z", "class std::basic_istream<char,struct std::char_traits<char> > & __cdecl std::operator>>(class std::basic_istream<char,struct std::char_traits<char> > &,unsigned char &)");

        /* 40 */
        // "__cdecl (*" vs "(__cdecl*"
        // expect("??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QAEAAV01@P6AAAVios_base@1@AAV21@@Z@Z", "public: class std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> > & __thiscall std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >::operator<<(class std::ios_base & (__cdecl*)(class std::ios_base &))");

        /* 41 */
        expect("??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QAEAAV01@PAV?$basic_streambuf@GU?$char_traits@G@std@@@1@@Z", "public: class std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> > & __thiscall std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >::operator<<(class std::basic_streambuf<unsigned short,struct std::char_traits<unsigned short> > *)");

        /* 42 */
        expect("??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QAEAAV01@PBX@Z", "public: class std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> > & __thiscall std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >::operator<<(void const *)");

        /* 43 */
        expect("??_8?$basic_fstream@DU?$char_traits@D@std@@@std@@7B?$basic_ostream@DU?$char_traits@D@std@@@1@@", "const std::basic_fstream<char,struct std::char_traits<char> >::`vbtable'{for `std::basic_ostream<char,struct std::char_traits<char> >'}");

        /* 44 */
        expect("??_8?$basic_fstream@GU?$char_traits@G@std@@@std@@7B?$basic_istream@GU?$char_traits@G@std@@@1@@", "const std::basic_fstream<unsigned short,struct std::char_traits<unsigned short> >::`vbtable'{for `std::basic_istream<unsigned short,struct std::char_traits<unsigned short> >'}");

        /* 45 */
        expect("??_8?$basic_fstream@GU?$char_traits@G@std@@@std@@7B?$basic_ostream@GU?$char_traits@G@std@@@1@@", "const std::basic_fstream<unsigned short,struct std::char_traits<unsigned short> >::`vbtable'{for `std::basic_ostream<unsigned short,struct std::char_traits<unsigned short> >'}");

        /* 46 */
        expect("??9std@@YA_NPBDABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@@Z", "bool __cdecl std::operator!=(char const *,class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        /* 47 */
        expect("??9std@@YA_NPBGABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@0@@Z", "bool __cdecl std::operator!=(unsigned short const *,class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");

        /* 48 */
        expect("??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEAADI@Z", "public: char & __thiscall std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> >::operator[](unsigned int)");

        /* 49 */
        expect("??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBEABDI@Z", "public: char const & __thiscall std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> >::operator[](unsigned int)const ");

        /* 50 */
        expect("??A?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEAAGI@Z", "public: unsigned short & __thiscall std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::operator[](unsigned int)");

        /* 51 */
        expect("??A?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBEABGI@Z", "public: unsigned short const & __thiscall std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::operator[](unsigned int)const ");

        /* 52 */
        expect(
            "?abs@std@@YAMABV?$complex@M@1@@Z",
            "float __cdecl std::abs(class std::complex<float> const &)",
        );

        /* 53 */
        expect(
            "?abs@std@@YANABV?$complex@N@1@@Z",
            "double __cdecl std::abs(class std::complex<double> const &)",
        );

        /* 54 */
        expect(
            "?abs@std@@YAOABV?$complex@O@1@@Z",
            "long double __cdecl std::abs(class std::complex<long double> const &)",
        );

        /* 55 */
        expect(
            "?cin@std@@3V?$basic_istream@DU?$char_traits@D@std@@@1@A",
            "class std::basic_istream<char,struct std::char_traits<char> > std::cin",
        );

        /* 56 */
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAG@Z", "protected: virtual class std::istreambuf_iterator<char,struct std::char_traits<char> > __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,unsigned short &)const ");

        /* 57 */
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAI@Z", "protected: virtual class std::istreambuf_iterator<char,struct std::char_traits<char> > __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,unsigned int &)const ");

        /* 58 */
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAJ@Z", "protected: virtual class std::istreambuf_iterator<char,struct std::char_traits<char> > __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,long &)const ");

        /* 59 */
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAK@Z", "protected: virtual class std::istreambuf_iterator<char,struct std::char_traits<char> > __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,unsigned long &)const ");

        /* 60 */
        expect("?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@V32@0AAVios_base@2@AAHAAM@Z", "protected: virtual class std::istreambuf_iterator<char,struct std::char_traits<char> > __thiscall std::num_get<char,class std::istreambuf_iterator<char,struct std::char_traits<char> > >::do_get(class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::istreambuf_iterator<char,struct std::char_traits<char> >,class std::ios_base &,int &,float &)const ");

        /* 61 */
        // "__cdecl (*" vs "(__cdecl*"
        // expect(
        //     "?_query_new_handler@@YAP6AHI@ZXZ",
        //     "int (__cdecl*__cdecl _query_new_handler(void))(unsigned int)",
        // );

        /* 62 */
        // "__cdecl (*" vs "(__cdecl*"
        // expect("?register_callback@ios_base@std@@QAEXP6AXW4event@12@AAV12@H@ZH@Z", "public: void __thiscall std::ios_base::register_callback(void (__cdecl*)(enum std::ios_base::event,class std::ios_base &,int),int)");

        /* 63 */
        expect("?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QAEAAV12@JW4seekdir@ios_base@2@@Z", "public: class std::basic_istream<char,struct std::char_traits<char> > & __thiscall std::basic_istream<char,struct std::char_traits<char> >::seekg(long,enum std::ios_base::seekdir)");

        /* 64 */
        expect("?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QAEAAV12@V?$fpos@H@2@@Z", "public: class std::basic_istream<char,struct std::char_traits<char> > & __thiscall std::basic_istream<char,struct std::char_traits<char> >::seekg(class std::fpos<int>)");

        /* 65 */
        expect("?seekg@?$basic_istream@GU?$char_traits@G@std@@@std@@QAEAAV12@JW4seekdir@ios_base@2@@Z", "public: class std::basic_istream<unsigned short,struct std::char_traits<unsigned short> > & __thiscall std::basic_istream<unsigned short,struct std::char_traits<unsigned short> >::seekg(long,enum std::ios_base::seekdir)");

        /* 66 */
        expect("?seekg@?$basic_istream@GU?$char_traits@G@std@@@std@@QAEAAV12@V?$fpos@H@2@@Z", "public: class std::basic_istream<unsigned short,struct std::char_traits<unsigned short> > & __thiscall std::basic_istream<unsigned short,struct std::char_traits<unsigned short> >::seekg(class std::fpos<int>)");

        /* 67 */
        expect("?seekoff@?$basic_filebuf@DU?$char_traits@D@std@@@std@@MAE?AV?$fpos@H@2@JW4seekdir@ios_base@2@H@Z", "protected: virtual class std::fpos<int> __thiscall std::basic_filebuf<char,struct std::char_traits<char> >::seekoff(long,enum std::ios_base::seekdir,int)");

        /* 68 */
        expect("?seekoff@?$basic_filebuf@GU?$char_traits@G@std@@@std@@MAE?AV?$fpos@H@2@JW4seekdir@ios_base@2@H@Z", "protected: virtual class std::fpos<int> __thiscall std::basic_filebuf<unsigned short,struct std::char_traits<unsigned short> >::seekoff(long,enum std::ios_base::seekdir,int)");

        /* 69 */
        // "__cdecl (*" vs "(__cdecl*"
        // expect(
        //     "?set_new_handler@@YAP6AXXZP6AXXZ@Z",
        //     "void (__cdecl*__cdecl set_new_handler(void (__cdecl*)(void)))(void)",
        // );

        /* 70 */
        expect("?str@?$basic_istringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z", "public: void __thiscall std::basic_istringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        /* 71 */
        expect("?str@?$basic_istringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ", "public: class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > __thiscall std::basic_istringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");

        /* 72 */
        expect("?str@?$basic_istringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z", "public: void __thiscall std::basic_istringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");

        /* 73 */
        expect("?str@?$basic_istringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ", "public: class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > __thiscall std::basic_istringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");

        /* 74 */
        expect("?str@?$basic_ostringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z", "public: void __thiscall std::basic_ostringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        /* 75 */
        expect("?str@?$basic_ostringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ", "public: class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > __thiscall std::basic_ostringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");

        /* 76 */
        expect("?str@?$basic_ostringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z", "public: void __thiscall std::basic_ostringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");

        /* 77 */
        expect("?str@?$basic_ostringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ", "public: class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > __thiscall std::basic_ostringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");

        /* 78 */
        expect("?str@?$basic_stringbuf@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z", "public: void __thiscall std::basic_stringbuf<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        /* 79 */
        expect("?str@?$basic_stringbuf@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ", "public: class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > __thiscall std::basic_stringbuf<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");

        /* 80 */
        expect("?str@?$basic_stringbuf@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z", "public: void __thiscall std::basic_stringbuf<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");

        /* 81 */
        expect("?str@?$basic_stringbuf@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ", "public: class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > __thiscall std::basic_stringbuf<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");

        /* 82 */
        expect("?str@?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@Z", "public: void __thiscall std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)");

        /* 83 */
        expect("?str@?$basic_stringstream@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QBE?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ", "public: class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > __thiscall std::basic_stringstream<char,struct std::char_traits<char>,class std::allocator<char> >::str(void)const ");

        /* 84 */
        expect("?str@?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QAEXABV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@@Z", "public: void __thiscall std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > const &)");

        /* 85 */
        expect("?str@?$basic_stringstream@GU?$char_traits@G@std@@V?$allocator@G@2@@std@@QBE?AV?$basic_string@GU?$char_traits@G@std@@V?$allocator@G@2@@2@XZ", "public: class std::basic_string<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> > __thiscall std::basic_stringstream<unsigned short,struct std::char_traits<unsigned short>,class std::allocator<unsigned short> >::str(void)const ");

        /* 86 */
        // missing private: static
        // expect(
        //     "?_Sync@ios_base@std@@0_NA",
        //     "private: static bool std::ios_base::_Sync",
        // );

        /* 87 */
        expect(
            "??_U@YAPAXI@Z",
            "void * __cdecl operator new[](unsigned int)",
        );

        /* 88 */
        expect("??_V@YAXPAX@Z", "void __cdecl operator delete[](void *)");

        /* 89 */
        expect("??X?$_Complex_base@M@std@@QAEAAV01@ABM@Z", "public: class std::_Complex_base<float> & __thiscall std::_Complex_base<float>::operator*=(float const &)");

        /* 90 */
        expect("??Xstd@@YAAAV?$complex@M@0@AAV10@ABV10@@Z", "class std::complex<float> & __cdecl std::operator*=(class std::complex<float> &,class std::complex<float> const &)");

        /* 91 */
        expect("?aaa@@YAHAAUbbb@@@Z", "int __cdecl aaa(struct bbb &)");

        /* 92 */
        expect(
            "?aaa@@YAHBAUbbb@@@Z",
            "int __cdecl aaa(struct bbb & volatile)",
        );

        /* 93 */
        expect("?aaa@@YAHPAUbbb@@@Z", "int __cdecl aaa(struct bbb *)");

        /* 94 */
        expect("?aaa@@YAHQAUbbb@@@Z", "int __cdecl aaa(struct bbb * const)");

        /* 95 */
        expect(
            "?aaa@@YAHRAUbbb@@@Z",
            "int __cdecl aaa(struct bbb * volatile)",
        );

        /* 96 */
        expect(
            "?aaa@@YAHSAUbbb@@@Z",
            "int __cdecl aaa(struct bbb * const volatile)",
        );

        /* 97 */
        // should fail to demangle because of the period, currently doesn't fail to demangle
        // expect("??0aa.a@@QAE@XZ", "??0aa.a@@QAE@XZ");

        /* 98 */
        expect(
            "??0aa$_3a@@QAE@XZ",
            "public: __thiscall aa$_3a::aa$_3a(void)",
        );

        /* 99 */
        expect("??2?$aaa@AAUbbb@@AAUccc@@AAU2@@ddd@1eee@2@QAEHXZ", "public: int __thiscall eee::eee::ddd::ddd::aaa<struct bbb &,struct ccc &,struct ccc &>::operator new(void)");

        /* 100 */
        // "__stdcall (*" vs "(__stdcall*"
        // expect("?pSW@@3P6GHKPAX0PAU_tagSTACKFRAME@@0P6GH0K0KPAK@ZP6GPAX0K@ZP6GK0K@ZP6GK00PAU_tagADDRESS@@@Z@ZA", "int (__stdcall* pSW)(unsigned long,void *,void *,struct _tagSTACKFRAME *,void *,int (__stdcall*)(void *,unsigned long,void *,unsigned long,unsigned long *),void * (__stdcall*)(void *,unsigned long),unsigned long (__stdcall*)(void *,unsigned long),unsigned long (__stdcall*)(void *,void *,struct _tagADDRESS *))");

        /* 101 */
        expect("?$_aaa@Vbbb@@", "_aaa<class bbb>");

        /* 102 */
        expect(
            "?$aaa@Vbbb@ccc@@Vddd@2@",
            "aaa<class ccc::bbb,class ccc::ddd>",
        );

        /* 103 */
        // "__stdcall (*" vs "(__stdcall*"
        // expect( "??0?$Foo@P6GHPAX0@Z@@QAE@PAD@Z", "public: __thiscall Foo<int (__stdcall*)(void *,void *)>::Foo<int (__stdcall*)(void *,void *)>(char *)");

        /* 104 */
        // "__stdcall (*" vs "(__stdcall*"
        // expect_with_flags( "??0?$Foo@P6GHPAX0@Z@@QAE@PAD@Z", "__thiscall Foo<int (__stdcall*)(void *,void *)>::Foo<int (__stdcall*)(void *,void *)>(char *)", 0x880);

        /* 105 */
        // "__cdecl (*" vs "(__cdecl*"
        // expect(
        //     "?Qux@Bar@@0PAP6AHPAV1@AAH1PAH@ZA",
        //     "private: static int (__cdecl** Bar::Qux)(class Bar *,int &,int &,int *)",
        // );

        /* 106 */
        // not respecting flags which presumably remove both return type and arguments
        // expect_with_flags("?Qux@Bar@@0PAP6AHPAV1@AAH1PAH@ZA", "Bar::Qux", 0x1800);

        /* 107 */
        expect("?$AAA@$DBAB@", "AAA<`template-parameter257'>");

        /* 108 */
        expect("?$AAA@?C@", "AAA<`template-parameter-2'>");

        /* 109 */
        expect("?$AAA@PAUBBB@@", "AAA<struct BBB *>");

        /* 110 */
        expect("??$ccccc@PAVaaa@@@bar@bb@foo@@DGPAV0@PAV0@PAVee@@IPAPAVaaa@@1@Z",
           "private: static class bar * __stdcall foo::bb::bar::ccccc<class aaa *>(class bar *,class ee *,unsigned int,class aaa * *,class ee *)");

        /* 111 */
        // missing "volatile "
        // expect(
        //     "?f@T@@QAEHQCY1BE@BO@D@Z",
        //     "public: int __thiscall T::f(char (volatile * const)[20][30])",
        // );

        /* 112 */
        expect(
            "?f@T@@QAEHQAY2BE@BO@CI@D@Z",
            "public: int __thiscall T::f(char (* const)[20][30][40])",
        );

        /* 113 */
        // double const
        // expect(
        //     "?f@T@@QAEHQAY1BE@BO@$$CBD@Z",
        //     "public: int __thiscall T::f(char const (* const)[20][30])",
        // );

        /* 114 */
        expect("??0?$Foo@U?$vector_c@H$00$01$0?1$0A@$0A@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@$0HPPPPPPP@@mpl@boost@@@@QAE@XZ",
           "public: __thiscall Foo<struct boost::mpl::vector_c<int,1,2,-2,0,0,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647> >::Foo<struct boost::mpl::vector_c<int,1,2,-2,0,0,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647,2147483647> >(void)");

        /* 115 */
        expect(
            "?swprintf@@YAHPAGIPBGZZ",
            "int __cdecl swprintf(unsigned short *,unsigned int,unsigned short const *,...)",
        );

        /* 116 */
        expect(
            "?vswprintf@@YAHPAGIPBGPAD@Z",
            "int __cdecl vswprintf(unsigned short *,unsigned int,unsigned short const *,char *)",
        );

        /* 117 */
        expect(
            "?vswprintf@@YAHPA_WIPB_WPAD@Z",
            "int __cdecl vswprintf(wchar_t *,unsigned int,wchar_t const *,char *)",
        );

        /* 118 */
        expect(
            "?swprintf@@YAHPA_WIPB_WZZ",
            "int __cdecl swprintf(wchar_t *,unsigned int,wchar_t const *,...)",
        );

        /* 119 */
        // missing __ptr64
        // expect("??Xstd@@YAAEAV?$complex@M@0@AEAV10@AEBV10@@Z", "class std::complex<float> & __ptr64 __cdecl std::operator*=(class std::complex<float> & __ptr64,class std::complex<float> const & __ptr64)");

        /* 120 */
        // missing __ptr64
        // expect(
        //     "?_Doraise@bad_cast@std@@MEBAXXZ",
        //     "protected: virtual void __cdecl std::bad_cast::_Doraise(void)const __ptr64",
        // );

        /* 121 */
        expect("??$?DM@std@@YA?AV?$complex@M@0@ABMABV10@@Z",
           "class std::complex<float> __cdecl std::operator*<float>(float const &,class std::complex<float> const &)");

        /* 122 */
        // missing const and missing __ptr64
        // expect("?_R2@?BN@???$_Fabs@N@std@@YANAEBV?$complex@N@1@PEAH@Z@4NB",
        //    "double const `double __cdecl std::_Fabs<double>(class std::complex<double> const & __ptr64,int * __ptr64)'::`29'::_R2");

        /* 123 */
        // fails with "B expected"
        // expect("?vtordisp_thunk@std@@$4PPPPPPPM@3EAA_NXZ",
        //    "[thunk]:public: virtual bool __cdecl std::vtordisp_thunk`vtordisp{4294967292,4}' (void) __ptr64");

        /* 124 */
        // missing final single quote
        // expect(
        //     "??_9CView@@$BBII@AE",
        //     "[thunk]: __thiscall CView::`vcall'{392,{flat}}' }'",
        // );

        /* 125 */
        // fails with "B expected"
        // expect("?_dispatch@_impl_Engine@SalomeApp@@$R4CE@BA@PPPPPPPM@7AE_NAAVomniCallHandle@@@Z",
        //    "[thunk]:public: virtual bool __thiscall SalomeApp::_impl_Engine::_dispatch`vtordispex{36,16,4294967292,8}' (class omniCallHandle &)");

        /* 126 */
        // has an additional "const " at the end, probably because it's ignoring the 0x60 flag
        // expect_with_flags(
        //     "?_Doraise@bad_cast@std@@MEBAXXZ",
        //     "protected: virtual void __cdecl std::bad_cast::_Doraise(void)",
        //     0x60,
        // );

        /* 127 */
        // missing some ptr64 (no underscores, maybe because of flag 1?)
        // expect_with_flags("??Xstd@@YAAEAV?$complex@M@0@AEAV10@AEBV10@@Z", "class std::complex<float> & ptr64 cdecl std::operator*=(class std::complex<float> & ptr64,class std::complex<float> const & ptr64)", 1);

        /* 128 */
        // has extra __cdecl in front of std::operator*=
        // expect("??Xstd@@YAAEAV?$complex@M@0@AEAV10@AEBV10@@Z",
        //    "class std::complex<float> & std::operator*=(class std::complex<float> &,class std::complex<float> const &)");

        /* 129 */
        // missing __ptr64
        // expect("??$run@XVTask_Render_Preview@@@QtConcurrent@@YA?AV?$QFuture@X@@PEAVTask_Render_Preview@@P82@EAAXXZ@Z",
        //    "class QFuture<void> __cdecl QtConcurrent::run<void,class Task_Render_Preview>(class Task_Render_Preview * __ptr64,void (__cdecl Task_Render_Preview::*)(void) __ptr64)");

        /* 130 */
        // missing space between char and [256]
        // expect("??_E?$TStrArray@$$BY0BAA@D$0BA@@@UAEPAXI@Z",
        //    "public: virtual void * __thiscall TStrArray<char [256],16>::`vector deleting destructor'(unsigned int)");
    }
}
