#include <wayland-server.h>

/*

void
wl_list_for_each(auto pos, auto head, auto link)
{
    for (pos = (__typeof__(pos))((char *)((&head.link)->next) - __builtin_offsetof (__typeof__(*pos), link));
        &pos->link != (&head.link);
        pos = (__typeof__(pos))((char *)(pos->link.next) - __builtin_offsetof (__typeof__(*pos), link)))
    {
        // code
    }
}

*/
