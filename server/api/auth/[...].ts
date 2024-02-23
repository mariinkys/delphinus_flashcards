// file: ~/server/api/auth/[...].ts
import { NuxtAuthHandler } from '#auth'
import bcrypt from 'bcryptjs'
import CredentialsProvider from 'next-auth/providers/credentials'
import prisma from '../../db/prisma'

export default NuxtAuthHandler({
    // A secret string you define, to ensure correct encryption
    secret: process.env.AUTH_SECRET,
    pages: {
        signIn: '/login'
    },
    providers: [
        // @ts-expect-error You need to use .default here for it to work during SSR. May be fixed via Vite at some point
        CredentialsProvider.default({
            name: 'Credentials',
            async authorize(credentials: any) {
                try {
                    const user = await prisma.user.findUnique({
                        where: {
                            username: credentials?.user,
                        },
                    })
                    if (user) {
                        if (credentials?.user === user.username) {
                            var ok = bcrypt.compareSync(credentials?.password, user.password);
                            if (ok) {
                                return user
                            } else {
                                return null
                            }
                        }
                    }
                } catch (error) {
                    console.log(error)
                    return null
                }
            }
        }),
    ]
})